use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Field(salsa::InternId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FieldData {
    pub name: Arc<String>,
    pub ty: TypeRef,
}

impl_intern!(Field, FieldData, intern_field, lookup_intern_field);
