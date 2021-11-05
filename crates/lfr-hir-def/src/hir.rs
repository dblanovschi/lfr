use std::sync::Arc;

use lfr_base_db::{impl_intern_key, salsa};

use crate::db;

trait Intern {
    type ID;
    fn intern(self, db: &dyn db::DefDatabase) -> Self::ID;
}

pub trait Lookup {
    type Data;
    fn lookup(&self, db: &dyn db::DefDatabase) -> Self::Data;
}

macro_rules! impl_intern {
    ($id:ident, $loc:ident, $intern:ident, $lookup:ident) => {
        impl_intern_key!($id);

        impl Intern for $loc {
            type ID = $id;

            fn intern(self, db: &dyn db::DefDatabase) -> $id {
                db.$intern(self)
            }
        }

        impl Lookup for $id {
            type Data = $loc;

            fn lookup(&self, db: &dyn db::DefDatabase) -> $loc {
                db.$lookup(*self)
            }
        }
    };
}

macro_rules! include_intern {
    ($vis:vis $name:ident, $f:expr) => {
        #[path = $f]
        mod $name;

        $vis use $name::*;
    };
}

include_intern!(pub struct_, "hir/struct.rs");
include_intern!(pub field, "hir/field.rs");
include_intern!(pub type_ref, "hir/type_ref.rs");
