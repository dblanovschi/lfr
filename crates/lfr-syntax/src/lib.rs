pub extern crate rowan;

pub mod ast;
pub mod span;
pub mod syntax_kind;

use rowan::Language;
pub use syntax_kind::SyntaxKind;

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct LfrLanguage;

impl Language for LfrLanguage {
    type Kind = SyntaxKind;

    fn kind_from_raw(raw: rowan::SyntaxKind) -> Self::Kind {
        assert!(raw.0 < SyntaxKind::__LAST as u16);
        unsafe { std::mem::transmute::<_, Self::Kind>(raw.0) }
    }

    fn kind_to_raw(kind: Self::Kind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(kind as u16)
    }
}

impl From<SyntaxKind> for rowan::SyntaxKind {
    fn from(kind: SyntaxKind) -> Self {
        LfrLanguage::kind_to_raw(kind)
    }
}

impl From<rowan::SyntaxKind> for SyntaxKind {
    fn from(kind: rowan::SyntaxKind) -> Self {
        LfrLanguage::kind_from_raw(kind)
    }
}

pub type SyntaxNode = rowan::SyntaxNode<LfrLanguage>;
pub type SyntaxToken = rowan::SyntaxToken<LfrLanguage>;
pub type NodeOrToken = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<LfrLanguage>;
