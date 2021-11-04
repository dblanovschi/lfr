use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TypeRef(salsa::InternId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TypeRefData {
    Never,
    Placeholder,
}

impl_intern!(TypeRef, TypeRefData, intern_type_ref, lookup_intern_type_ref);
