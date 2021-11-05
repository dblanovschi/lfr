pub use salsa::{self, Cancelled};

#[macro_export]
macro_rules! impl_intern_key {
    ($name:ident) => {
        impl $crate::salsa::InternKey for $name {
            fn from_intern_id(v: $crate::salsa::InternId) -> Self {
                $name(v)
            }

            fn as_intern_id(&self) -> $crate::salsa::InternId {
                self.0
            }
        }
    };
}
