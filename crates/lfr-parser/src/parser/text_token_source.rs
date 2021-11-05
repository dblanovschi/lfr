use lfr_stdx::TakeIfUnless;
use lfr_syntax::rowan::TextSize;
use lfr_syntax::span::Span;
use lfr_syntax::syntax_kind::SyntaxKind::EOF;
use lfr_syntax::{SyntaxKind, T};

use super::{FindProperty, ForwardToken, Token, TokenSource};
use crate::parser::IsTrivia;

///
#[derive(Debug)]
pub struct LexerWrap {
    pos: usize,
    tokens: Vec<(Token, TextSize)>,
}

impl LexerWrap {
    /// Creates a new `LexerWrap` from the list of tokens and starts at token
    /// indexed `0`
    pub fn new(tokens: &[Token]) -> Self {
        let mut text_off = 0.into();
        let tokens = tokens
            .iter()
            .copied()
            .filter_map(|it| {
                let tk = (it, text_off);
                text_off += it.len;
                tk.take_unless(|tk| tk.0.syntax_kind.is_trivia())
            })
            .collect();
        Self { pos: 0, tokens }
    }
}

impl TokenSource for LexerWrap {
    fn current(&self) -> Token {
        self.lookahead(0)
    }

    fn lookahead(&self, n: usize) -> Token {
        self.tokens
            .get(self.pos + n)
            .copied()
            .map(|it| it.0)
            .unwrap_or(Token {
                syntax_kind: EOF,
                len: Span::from(0..0).text_range.len(),
            })
    }

    fn bump(&mut self) {
        if self.pos < self.tokens.len() {
            self.pos += 1;
        }
    }

    fn find(&self, find_property: FindProperty) -> ForwardToken {
        const NOT_FOUND: ForwardToken = ForwardToken {
            kind: EOF,
            offset: 0,
            state: 0,
        };
        if self.tokens.len() <= self.pos {
            return NOT_FOUND;
        }

        fn at_composite2(
            tokens: &[(Token, TextSize)],
            kind1: SyntaxKind,
            kind2: SyntaxKind,
        ) -> bool {
            tokens.len() >= 2
                && tokens[0].0.syntax_kind == kind1
                && tokens[1].0.syntax_kind == kind2
        }

        fn at_composite3(
            tokens: &[(Token, TextSize)],
            kind1: SyntaxKind,
            kind2: SyntaxKind,
            kind3: SyntaxKind,
        ) -> bool {
            tokens.len() >= 3
                && tokens[0].0.syntax_kind == kind1
                && tokens[1].0.syntax_kind == kind2
                && tokens[2].0.syntax_kind == kind3
        }

        fn at(tokens: &[(Token, TextSize)], kind: SyntaxKind) -> bool {
            // TAG: composites
            match kind {
                T![&&] => at_composite2(tokens, T![&], T![&]),
                T![||] => at_composite2(tokens, T![|], T![|]),
                T![+=] => at_composite2(tokens, T![+], T![=]),
                T![-=] => at_composite2(tokens, T![-], T![=]),
                T![*=] => at_composite2(tokens, T![*], T![=]),
                T![/=] => at_composite2(tokens, T![/], T![=]),
                T![%=] => at_composite2(tokens, T![%], T![=]),
                T![&=] => at_composite2(tokens, T![&], T![=]),
                T![|=] => at_composite2(tokens, T![|], T![=]),
                T![^=] => at_composite2(tokens, T![^], T![=]),
                T![&&=] => at_composite3(tokens, T![&], T![&], T![=]),
                T![||=] => at_composite3(tokens, T![|], T![|], T![=]),
                T![==] => at_composite2(tokens, T![=], T![=]),
                T![!=] => at_composite2(tokens, T![!], T![=]),
                T![<=] => at_composite2(tokens, T![<], T![=]),
                T![>=] => at_composite2(tokens, T![>], T![=]),
                T![::] => at_composite2(tokens, T![:], T![:]),
                kind => tokens[0].0.syntax_kind == kind,
            }
        }

        match find_property {
            FindProperty::In(kinds) => {
                for tind in self.pos..self.tokens.len() {
                    let t = &self.tokens[tind..];

                    if let Some(&kind) = kinds.iter().find(|&&it| at(t, it)) {
                        return ForwardToken {
                            kind,
                            offset: tind - self.pos,
                            state: self.pos,
                        };
                    }
                }

                NOT_FOUND
            }
            FindProperty::NotIn(kinds) => {
                for tind in self.pos..self.tokens.len() {
                    let t = &self.tokens[tind..];

                    if let None = kinds.iter().find(|&&it| at(t, it)) {
                        return ForwardToken {
                            kind: t[0].0.syntax_kind,
                            offset: tind - self.pos,
                            state: self.pos,
                        };
                    }
                }

                NOT_FOUND
            }
            FindProperty::KindIs(kind) => {
                for tind in self.pos..self.tokens.len() {
                    let t = &self.tokens[tind..];

                    if at(t, kind) {
                        return ForwardToken {
                            kind: t[0].0.syntax_kind,
                            offset: tind - self.pos,
                            state: self.pos,
                        };
                    }
                }

                NOT_FOUND
            }
            FindProperty::KindIsNot(kind) => {
                for tind in self.pos..self.tokens.len() {
                    let t = &self.tokens[tind..];

                    if !at(t, kind) {
                        return ForwardToken {
                            kind: t[0].0.syntax_kind,
                            offset: tind - self.pos,
                            state: self.pos,
                        };
                    }
                }

                NOT_FOUND
            }
        }
    }

    fn bump_to(&self, forward_token: ForwardToken) -> usize {
        if forward_token.kind == EOF {
            return 0;
        }

        forward_token.state + forward_token.offset - self.pos
    }
}
