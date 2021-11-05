use std::convert::TryInto;
use std::ops::Range;

use lfr_stdx::Let;
use lfr_syntax::span::Span;
use lfr_syntax::syntax_kind::SyntaxKind;
use lfr_syntax::T;
use logos::Logos;

#[derive(Logos, Copy, Clone, Debug, PartialEq, PartialOrd, Eq)]
pub enum Tk {
    #[regex(r#"[a-zA-Z_][a-zA-Z0-9_]*"#)]
    Ident,
    #[regex(r#"([1-9][0-9]*|0x[0-9a-fA-F]+|0b[01]+|0[0-7]+|0)[uU]?[lL]?"#)]
    IntNumber,
    #[regex(r#"'(\\['nt\\]|[^'\\])+'"#)]
    Str,
    #[regex(r#"'''([^']*|'[^']|''[^'])*'''"#)]
    MultilineStr,
    #[regex(r#"//[^\n]*"#)]
    Comment,
    #[regex(r#"/\*([^*]|\**[^*/])*\*+/"#)]
    BlockComment,
    #[regex(r#"[ \t\r]+"#)]
    Whitespace,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Asterisk,
    #[token("/")]
    Slash,
    #[token("%")]
    Percent,
    #[token(">")]
    RAngle,
    #[token("<")]
    LAngle,
    #[token("=")]
    Eq,
    #[token("!")]
    Bang,
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token(".")]
    Dot,
    #[token(":")]
    Colon,
    #[token("?")]
    QMark,
    #[token(";")]
    Semicolon,
    #[token(",")]
    Comma,
    #[token("in")]
    InKw,
    #[token("let")]
    LetKw,
    #[token("if")]
    IfKw,
    #[token("else")]
    ElseKw,
    #[token("foreach")]
    ForeachKw,
    #[token("continue")]
    ContinueKw,
    #[token("break")]
    BreakKw,
    #[token("return")]
    ReturnKw,
    #[token("true")]
    TrueKw,
    #[token("false")]
    FalseKw,
    #[token("fn")]
    FnKw,
    #[token("import")]
    ImportKw,
    #[token("\n")]
    Newline,
    #[error]
    #[regex(r"/\*([^*]|\*+[^*/])*\*?")]
    Error,
}
impl From<Tk> for SyntaxKind {
    fn from(tk: Tk) -> Self {
        use SyntaxKind::*;
        match tk {
            Tk::Ident => T![ident],
            Tk::IntNumber => T![int_number],
            Tk::Str => T![str],
            Tk::MultilineStr => T![multiline_str],
            Tk::Comment => T![comment],
            Tk::BlockComment => T![block_comment],
            Tk::Whitespace => T![whitespace],
            Tk::Plus => T ! [+],
            Tk::Minus => T ! [-],
            Tk::Asterisk => T ! [*],
            Tk::Slash => T ! [/],
            Tk::Percent => T ! [%],
            Tk::RAngle => T ! [>],
            Tk::LAngle => T ! [<],
            Tk::Eq => T ! [=],
            Tk::Bang => T![!],
            Tk::LParen => T!['('],
            Tk::RParen => T![')'],
            Tk::LBracket => T!['['],
            Tk::RBracket => T![']'],
            Tk::LBrace => T!['{'],
            Tk::RBrace => T!['}'],
            Tk::Dot => T ! [.],
            Tk::Colon => T ! [:],
            Tk::QMark => T ! [?],
            Tk::Semicolon => T ! [;],
            Tk::Comma => T ! [,],
            Tk::InKw => T![in],
            Tk::LetKw => T![let],
            Tk::IfKw => T![if],
            Tk::ElseKw => T![else],
            Tk::ForeachKw => T![for],
            Tk::ContinueKw => T![continue],
            Tk::BreakKw => T![break],
            Tk::ReturnKw => T![return],
            Tk::TrueKw => T![true],
            Tk::FalseKw => T![false],
            Tk::FnKw => T![fn],
            Tk::ImportKw => T![import],
            Tk::Newline => T![newline],
            Tk::Error => ERROR,
        }
    }
}
#[allow(missing_debug_implementations)]
pub struct Lexer<'a> {
    lexer: logos::SpannedIter<'a, Tk>,
}
impl<'a> Lexer<'a> {
    pub(crate) fn new(s: &'a str) -> Self {
        let lexer = Tk::lexer(s).spanned();
        Self { lexer }
    }
}
impl<'a> Iterator for Lexer<'a> {
    type Item = (SyntaxKind, Span);

    fn next(&mut self) -> Option<Self::Item> {
        self.lexer.next().map(|(token, span)| {
            (
                token.into(),
                span.let_(|it| -> Range<u32> {
                    it.start.try_into().unwrap()..it.end.try_into().unwrap()
                })
                .into(),
            )
        })
    }
}
