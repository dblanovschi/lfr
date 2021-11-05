use std::fmt;
use std::ops::Range;

use rowan::{TextRange, TextSize};

/// A span in the source code
#[derive(Copy, Clone, Default, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct Span {
    /// The underlying text range
    pub text_range: TextRange,
}

impl From<Range<TextSize>> for Span {
    fn from(range: Range<TextSize>) -> Self {
        Self {
            text_range: TextRange::new(range.start, range.end),
        }
    }
}

impl From<Range<u32>> for Span {
    fn from(range: Range<u32>) -> Self {
        Self {
            text_range: TextRange::new(range.start.into(), range.end.into()),
        }
    }
}

impl fmt::Debug for Span {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.text_range.fmt(f)
    }
}
