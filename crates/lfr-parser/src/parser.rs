pub mod error;
mod event;
mod marker;
pub mod text_token_source;
pub mod text_tree_sink;
pub mod token_source;
pub mod tree_sink;

use lfr_stdx::CopyTo;
use lfr_syntax::rowan::{GreenNode, NodeOrToken, TextSize};
use lfr_syntax::SyntaxKind::{self, *};
use lfr_syntax::{SyntaxNode, T};
use text_token_source::LexerWrap;

use self::error::ParseError;
use self::event::Event;
use self::marker::{CompletedMarker, Marker};
use self::text_tree_sink::TextTreeSink;
use self::token_source::{FindProperty, ForwardToken, Token, TokenSource};
use self::tree_sink::TreeSink;
use crate::lexer::Lexer;

trait IsTrivia: Copy {
    fn is_trivia(self) -> bool;
}

impl IsTrivia for SyntaxKind {
    fn is_trivia(self) -> bool {
        self == Self::WHITESPACE
            || self == Self::COMMENT
            || self == Self::BLOCK_COMMENT
    }
}

pub(crate) struct Parser<'ts> {
    /// tokens
    source: &'ts mut dyn TokenSource,
    events: Vec<Event>,
}

#[allow(clippy::inline_always)]
impl<'ts> Parser<'ts> {
    fn parse(&mut self) {
        parse_root(self);
    }

    #[inline(always)]
    fn at(&self, kind: SyntaxKind) -> bool {
        // TAG: composites
        match kind {
            T![&&] => self.at_composite2(T![&], T![&]),
            T![||] => self.at_composite2(T![|], T![|]),
            T![+=] => self.at_composite2(T![+], T![=]),
            T![-=] => self.at_composite2(T![-], T![=]),
            T![*=] => self.at_composite2(T![*], T![=]),
            T![/=] => self.at_composite2(T![/], T![=]),
            T![%=] => self.at_composite2(T![%], T![=]),
            T![&=] => self.at_composite2(T![&], T![=]),
            T![|=] => self.at_composite2(T![|], T![=]),
            T![^=] => self.at_composite2(T![^], T![=]),
            T![&&=] => self.at_composite3(T![&], T![&], T![=]),
            T![||=] => self.at_composite3(T![|], T![|], T![=]),
            T![==] => self.at_composite2(T![=], T![=]),
            T![!=] => self.at_composite2(T![!], T![=]),
            T![<=] => self.at_composite2(T![<], T![=]),
            T![>=] => self.at_composite2(T![>], T![=]),
            T![::] => self.at_composite2(T![:], T![:]),
            kind => self.current() == kind,
        }
    }

    fn at_composite2(&self, kind1: SyntaxKind, kind2: SyntaxKind) -> bool {
        self.current() == kind1 && self.nth(1) == kind2
    }

    fn at_composite3(
        &self,
        kind1: SyntaxKind,
        kind2: SyntaxKind,
        kind3: SyntaxKind,
    ) -> bool {
        self.current() == kind1 && self.nth(1) == kind2 && self.nth(2) == kind3
    }

    #[inline(always)]
    fn at_any<I>(&self, kinds: I) -> Option<SyntaxKind>
    where
        I: IntoIterator<Item = SyntaxKind>,
    {
        kinds.into_iter().find(|it| self.at(*it))
    }

    #[inline(always)]
    fn bump(&mut self, kind: SyntaxKind) {
        assert!(self.eat(kind));
    }

    #[inline(always)]
    fn bump_any(&mut self) {
        let kind = self.current();
        if kind == EOF {
            return;
        }

        self.do_bump(kind, Parser::sk_raw_tokens(kind));
    }

    fn do_bump(&mut self, kind: SyntaxKind, n_raw_tokens: u8) {
        assert!(n_raw_tokens <= 3);
        for _ in 0..n_raw_tokens {
            self.source.bump();
        }
        self.push_event(Event::Token { kind, n_raw_tokens })
    }

    fn expect_failed(&mut self, kind: SyntaxKind) {
        self.error(format!("expected {:?}, got {:?}", kind, self.current()));
    }

    fn expect(&mut self, kind: SyntaxKind) -> bool {
        if self.eat(kind) {
            return true;
        }
        self.expect_failed(kind);
        false
    }

    pub(crate) fn error<E: Into<ParseError>>(&mut self, msg: E) {
        let msg = msg.into();
        self.push_event(Event::Error { msg })
    }

    #[inline(always)]
    fn sk_raw_tokens(kind: SyntaxKind) -> u8 {
        // TAG: composites
        match kind {
            T![&&] => 2,
            T![||] => 2,
            T![+=] => 2,
            T![-=] => 2,
            T![*=] => 2,
            T![/=] => 2,
            T![%=] => 2,
            T![&=] => 2,
            T![|=] => 2,
            T![^=] => 2,
            T![&&=] => 3,
            T![||=] => 3,
            T![==] => 2,
            T![!=] => 2,
            T![<=] => 2,
            T![>=] => 2,
            T![::] => 2,
            _ => 1,
        }
    }

    #[inline(always)]
    fn eat(&mut self, kind: SyntaxKind) -> bool {
        if self.at(kind) {
            self.do_bump(kind, Parser::sk_raw_tokens(kind));
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn eat_any<I>(&mut self, kinds: I) -> bool
    where
        I: IntoIterator<Item = SyntaxKind>,
    {
        if let Some(kind) = self.at_any(kinds) {
            self.do_bump(kind, Parser::sk_raw_tokens(kind));
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    #[inline(always)]
    fn nth(&self, n: usize) -> SyntaxKind {
        self.source.lookahead(n).syntax_kind
    }

    #[inline(always)]
    fn require_newline(&mut self) {
        if self.eat(T![newline]) {
            return;
        }

        if self.at(EOF) {
            return;
        }

        self.expect_failed(T![newline]);
    }

    #[inline(always)]
    fn has_newline(&self) -> bool {
        self.at(T![newline])
    }

    #[inline(always)]
    fn bump_to(&mut self, forward_token: ForwardToken) {
        let bumps = self.source.bump_to(forward_token);
        for _ in 0..bumps {
            self.bump_any();
        }
    }

    #[inline(always)]
    fn next_not_newline(&self) -> ForwardToken {
        self.source.find(FindProperty::KindIsNot(T![newline]))
    }

    #[inline(always)]
    fn bump_to_if_next_non_newline_is(&mut self, kind: SyntaxKind) -> bool {
        let mut tk = ForwardToken::default();
        let k = self.next_not_newline().copy_to(&mut tk).kind;
        if k == kind {
            self.bump_to(tk);
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn bump_to_if_next_non_newline_is_any<I>(&mut self, kinds: I) -> bool
    where
        I: IntoIterator<Item = SyntaxKind>,
    {
        let mut tk = ForwardToken::default();
        let k = self.next_not_newline().copy_to(&mut tk).kind;
        if kinds.into_iter().any(|it| it == k) {
            self.bump_to(tk);
            true
        } else {
            false
        }
    }

    #[inline(always)]
    fn skip_newlines(&mut self) {
        while self.at(T![newline]) {
            self.do_bump(T![newline], 1);
        }
    }

    #[inline(always)]
    fn unexpected(&mut self) {
        let current = self.current();
        if current != EOF {
            self.do_bump(current, Parser::sk_raw_tokens(current));
        }

        self.error(format!("Unexpected {:?}", current));
    }

    #[inline(always)]
    fn start(&mut self) -> Marker {
        let pos = self.events.len() as u32;
        self.push_event(Event::tombstone());
        Marker::new(pos)
    }

    #[inline(always)]
    fn push_event(&mut self, event: Event) {
        self.events.push(event)
    }
}
///
pub fn parse(text: &str) -> (GreenNode, Vec<(ParseError, TextSize)>) {
    let lexer = Lexer::new(text);
    let tokens: Vec<_> = lexer
        .map(|(kind, span)| Token {
            syntax_kind: kind,
            len: span.text_range.len(),
        })
        .collect();
    let mut lexer = LexerWrap::new(&tokens);
    let mut sink = TextTreeSink::new(text, &tokens);

    parse_to_sink(&mut lexer, &mut sink);
    sink.finish()
}

/// parses `text`
pub fn parse_to_sink(source: &mut dyn TokenSource, sink: &mut dyn TreeSink) {
    let mut p = Parser {
        source,
        events: vec![],
    };
    p.parse();
    event::process(sink, p.events);
}

fn parse_root(p: &mut Parser) {
    let marker = p.start();

    loop {
        p.skip_newlines();
        if p.at(EOF) || !p.at(T![import]) {
            break;
        }
        parse_import_stmt(p);
    }

    loop {
        p.skip_newlines();
        if p.at(EOF) {
            break;
        }
        parse_lang_item(p);
    }

    marker.complete(p, ROOT);
}

fn parse_import_stmt(p: &mut Parser) {
    let marker = p.start();

    p.expect(T![import]);

    parse_import_target(p);

    if p.at(T![as]) {
        p.bump(T![as]);
        parse_ident(p);
    }

    p.require_newline();

    marker.complete(p, IMPORT_STMT);
}

fn parse_import_target(p: &mut Parser) {
    let marker = p.start();
    if is_string_lit(p) {
        parse_string(p);
    } else {
        parse_ident(p);
    }

    marker.complete(p, IMPORT_TARGET);
}

fn parse_ident(p: &mut Parser) {
    p.expect(IDENT);
}

fn parse_lang_item(p: &mut Parser) {
    p.skip_newlines();
    parse_statement(p);
}

fn parse_statement(p: &mut Parser) {
    let tok = p.current();

    let statement_marker = p.start();

    match tok {
        T!['(']
        | T!['[']
        | T!['{']
        | T![+]
        | T![-]
        | T![!]
        | T![true]
        | T![false]
        | T![int_number]
        // | T![float_number]
        | T![ident]
        | T![str]
        | T![multiline_str]
        | T![if] => {
            parse_expr(p);
        }
        T![let] => {
            parse_declaration(p);
        }
        T![for] => {
            parse_for(p);
        }
        T![while] => {
            parse_while(p);
        }
        T![continue] | T![break] | T![return] => {
            parse_control_stmt(p);
        }
        _ => {
            p.unexpected();
        }
    }

    statement_marker.complete(p, STMT);
}

fn parse_expr(p: &mut Parser) {
    // test precedence_parsing
    // x = 1 + 2 * 3 % - 4 ( 5 )

    parse_precedence_9_expr(p);
}

fn parse_tuple_expr(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();
    assert!(is_tuple_expr_start(p));

    parse_tt(p, TUPLE_EXPR, T!['('], Some(T![,]), T![')'], parse_expr)
}

fn is_tuple_expr_start(p: &mut Parser) -> bool {
    p.at(T!['('])
}

fn parse_array_expr(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();
    assert!(is_array_expr_start(p));

    parse_tt(p, ARR_EXPR, T!['['], Some(T![,]), T![']'], parse_expr)
}

fn is_array_expr_start(p: &mut Parser) -> bool {
    p.at(T!['['])
}

fn parse_primary(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();

    let mk = p.start();

    if is_array_expr_start(p) {
        parse_array_expr(p);
    } else if is_tuple_expr_start(p) {
        parse_tuple_expr(p);
    } else if is_conditional_start(p) {
        // test if_condition_in_expr
        // a = if b {} else {}

        // test if_condition_in_expr_stretched
        // a = if b
        // {
        //
        // }
        //
        // else
        // {
        //
        //
        // }
        parse_conditional(p);
    } else if is_expr_block_start(p) {
        parse_expr_block(p);
    } else if p
        .at_any([
            T![int_number],
            // T![float_number],
            T![ident],
        ])
        .is_some()
    {
        p.bump_any();
    } else if is_string_lit(p) {
        parse_string(p);
    } else {
        p.unexpected();
    }

    mk.complete(p, PRIMARY_EXPR)
}

fn is_string_lit(p: &mut Parser) -> bool {
    p.at_any([T![str], T![multiline_str]]).is_some()
}

fn parse_string(p: &mut Parser) {
    assert!(is_string_lit(p));
    p.bump_any();
}

fn parse_tt(
    p: &mut Parser,
    outer_kind: SyntaxKind,
    start_tok: SyntaxKind,
    separator: Option<SyntaxKind>,
    end_tok: SyntaxKind,
    mut f: impl FnMut(&mut Parser),
) -> CompletedMarker {
    let marker = p.start();
    p.bump(start_tok);

    p.skip_newlines();
    while !p.at_any([EOF, end_tok]).is_some() {
        f(p);

        if let Some(separator) = separator {
            p.skip_newlines();
            if p.eat(separator) {
            } else if !p.at(end_tok) {
                p.error(format!(
                    "expected {:?} or {:?}, got {:?}",
                    separator,
                    end_tok,
                    p.current()
                ));
            }
        }
        p.skip_newlines();
    }

    p.expect(end_tok);
    marker.complete(p, outer_kind)
}

fn parse_precedence_1_expr(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();
    let mut marker = parse_primary(p);

    // test index_on_second_line_is_array_lit_expr
    // x = a
    // [1]

    loop {
        if let Some(kind) = p.at_any([T!['('], T!['[']]) {
            if kind == T!['('] {
                let new_marker = marker.precede(p);
                marker = parse_f_call(p, new_marker);
            } else if kind == T!['['] {
                let new_marker = marker.precede(p);
                marker = parse_index_expr(p, new_marker);
            } else if kind == T![.] {
                let new_marker = marker.precede(p);
                marker = parse_member_expr(p, new_marker);

                if p.at(T!['(']) {
                    let m = marker.undo_completion(p);

                    marker = parse_method_call(p, m);
                }
            }
        } else if p.bump_to_if_next_non_newline_is(T![.]) {
            let new_marker = marker.precede(p);
            marker = parse_member_expr(p, new_marker);

            if p.at(T!['(']) {
                let m = marker.undo_completion(p);

                marker = parse_method_call(p, m);
            }
        } else {
            break;
        }
    }

    marker
}

fn parse_f_call(p: &mut Parser, marker: Marker) -> CompletedMarker {
    // test simple_function_call
    // x = f(1, 2, a = b)

    // test function_call
    // x = f(1, 2, a = b)

    // test named_args_only_function_call
    // x = f(a = b, c = d)

    // test err func_call_on_second_line
    // x = f
    // (1, 2, a = b)

    parse_tt(p, FN_CALL_ARGS, T!['('], Some(T![,]), T![')'], parse_farg);
    marker.complete(p, FN_CALL_EXPR)
}

fn parse_farg(p: &mut Parser) {
    p.skip_newlines();

    parse_expr(p)
}

fn parse_index_expr(p: &mut Parser, marker: Marker) -> CompletedMarker {
    let brackets_marker = p.start();
    p.bump(T!['[']);
    parse_expr(p);
    p.expect(T![']']);
    brackets_marker.complete(p, INDEX_EXPR_BRACKETS);
    marker.complete(p, INDEX_EXPR)
}

fn parse_member_expr(p: &mut Parser, marker: Marker) -> CompletedMarker {
    p.bump(T![.]);
    p.expect(T![ident]);

    marker.complete(p, MEMBER_ACCESS_EXPR)
}

fn parse_method_call(p: &mut Parser, marker: Marker) -> CompletedMarker {
    parse_tt(p, FN_CALL_ARGS, T!['('], Some(T![,]), T![')'], parse_farg);

    marker.complete(p, METHOD_CALL_EXPR)
}

fn parse_precedence_2_expr(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();
    if let Some(kind) = p.at_any([T![+], T![-], T![!]]) {
        let marker = p.start();
        p.bump(kind);
        parse_precedence_2_expr(p);
        marker.complete(p, PREFIX_UNARY_EXPR)
    } else {
        parse_precedence_1_expr(p)
    }
}

fn parse_infix_binop<I>(
    p: &mut Parser,
    ops: I,
    mut lower: impl FnMut(&mut Parser) -> CompletedMarker,
) -> CompletedMarker
where
    I: IntoIterator<Item = SyntaxKind> + Copy,
{
    p.skip_newlines();
    let mut completed = lower(p);

    // test expr_with_binary_infix_operators_on_same_line_and_second_operand_on_second_line
    // x = 1 +
    // 2 +
    // f(
    //      4
    // )
    // y = 3 *
    // 6 %
    // 78

    // test expr_with_binary_infix_operators_on_next_line
    // x = 1
    // + 2
    // + f(
    //      4
    // )
    // y = 3
    // * 6
    // % 78

    while let Some(kind) = p.at_any(ops) {
        let prec = completed.precede(p);
        p.bump(kind);
        lower(p);

        completed = prec.complete(p, BIN_EXPR);
    }

    completed
}

fn parse_precedence_3_expr(p: &mut Parser) -> CompletedMarker {
    parse_infix_binop(p, [T![*], T![/], T![%]], parse_precedence_2_expr)
}

fn parse_precedence_4_expr(p: &mut Parser) -> CompletedMarker {
    parse_infix_binop(p, [T![+], T![-]], parse_precedence_3_expr)
}

fn parse_precedence_5_expr(p: &mut Parser) -> CompletedMarker {
    parse_infix_binop(
        p,
        [T![<], T![<=], T![>], T![>=]],
        parse_precedence_4_expr,
    )
}

fn parse_precedence_6_expr(p: &mut Parser) -> CompletedMarker {
    parse_infix_binop(p, [T![==], T![!=]], parse_precedence_5_expr)
}

fn parse_precedence_7_expr(p: &mut Parser) -> CompletedMarker {
    parse_infix_binop(p, [T![&&]], parse_precedence_6_expr)
}

fn parse_precedence_8_expr(p: &mut Parser) -> CompletedMarker {
    parse_infix_binop(p, [T![||]], parse_precedence_7_expr)
}

fn parse_precedence_9_expr(p: &mut Parser) -> CompletedMarker {
    parse_infix_binop(
        p,
        [
            T![=],
            T![+=],
            T![-=],
            T![*=],
            T![/=],
            T![%=],
            T![&=],
            T![|=],
            T![^=],
            T![&&=],
            T![||=],
        ],
        parse_precedence_8_expr,
    )
}

fn parse_expr_block(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();
    parse_tt(p, BLOCK, T!['{'], None, T!['}'], parse_statement)
}

fn is_expr_block_start(p: &mut Parser) -> bool {
    p.at(T!['{'])
}

fn parse_declaration(p: &mut Parser) -> CompletedMarker {
    // test declaration
    // let a = b

    // test var_declaration_with_proper_expr_as_value
    // let x = 1
    p.skip_newlines();
    let mk = p.start();
    p.expect(T![let]);
    p.expect(T![ident]);

    p.expect(T![=]);

    parse_expr(p);

    p.require_newline();

    mk.complete(p, DECLARATION_STMT)
}

fn is_conditional_start(p: &mut Parser) -> bool {
    p.at(T![if])
}

fn parse_conditional(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();

    // test if_else_condition
    // if a {} else {}

    // test if_else_if_else_condition
    // if a {} else if b {} else {}

    assert!(is_conditional_start(p));

    let marker = p.start();
    parse_conditional_branch(p);
    // test if_else_if_else_condition_stretched
    // if ()
    //
    // {
    //
    // }
    //
    // else if
    // ()
    //
    // {
    //
    // }
    //
    // else {
    //
    // }

    // test if_condition_on_next_line
    // if
    //      1
    // {}
    while p.bump_to_if_next_non_newline_is(T![else]) {
        p.bump(T![else]);
        if p.bump_to_if_next_non_newline_is(T![if]) {
            parse_conditional_branch(p);
        } else {
            parse_expr_block(p);
            break;
        }
    }

    marker.complete(p, IF_EXPR)
}

fn parse_conditional_branch(p: &mut Parser) {
    p.skip_newlines();

    p.bump(T![if]);

    parse_expr(p);

    parse_expr_block(p);
}

fn parse_for(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();

    // test for_basic
    // for a in b {}

    let marker = p.start();

    p.bump(T![for]);
    parse_for_in_expr(p);
    parse_expr_block(p);

    p.require_newline();

    marker.complete(p, FOR_STMT)
}

fn parse_for_in_expr(p: &mut Parser) -> CompletedMarker {
    let marker = p.start();
    parse_expr(p);
    p.expect(T![in]);
    parse_expr(p);
    marker.complete(p, FOR_IN_EXPR)
}

fn parse_while(p: &mut Parser) -> CompletedMarker {
    p.skip_newlines();

    // test while_basic
    // while a {}

    let marker = p.start();

    p.bump(T![while]);
    parse_expr(p);
    parse_expr_block(p);

    p.require_newline();

    marker.complete(p, FOR_STMT)
}

fn is_control_stmt(p: &mut Parser) -> bool {
    p.at_any([T![continue], T![break], T![return]]).is_some()
}

fn parse_control_stmt(p: &mut Parser) -> CompletedMarker {
    // test continue_test
    // continue

    // test break_without_value
    // break

    // test break_with_value
    // break 1

    // test break_with_value_on_new_line
    // break
    //        1

    // test return_without_value
    // return

    // test return_with_value
    // return 1

    // test return_with_value_on_new_line
    // return
    //         1
    p.skip_newlines();
    assert!(is_control_stmt(p));

    let marker = p.start();
    if p.eat(T![continue]) {
        p.require_newline();
        marker.complete(p, CONTINUE_STMT)
    } else if p.eat(T![return]) {
        if !p.at(T![newline]) {
            parse_expr(p);
        }
        p.require_newline();
        marker.complete(p, RETURN_STMT)
    } else if p.eat(T![break]) {
        if !p.at(T![newline]) {
            parse_expr(p);
        }
        p.require_newline();
        marker.complete(p, BREAK_STMT)
    } else {
        unreachable!()
    }
}

pub fn print_ast(node: &SyntaxNode, ident: usize) {
    println!(
        "{:ident$}{:?}@{:?}..{:?}",
        "",
        node.kind(),
        node.text_range().start(),
        node.text_range().end(),
        ident = ident * 2
    );
    for child in node.children_with_tokens() {
        match child {
            NodeOrToken::Node(n) => {
                print_ast(&n, ident + 1);
            }
            NodeOrToken::Token(t) => {
                println!(
                    "{:ident$}{:?}@{:?}..{:?} {:?}",
                    "",
                    t.kind(),
                    t.text_range().start(),
                    t.text_range().end(),
                    t.text(),
                    ident = (ident + 1) * 2
                );
            }
        }
    }
}
