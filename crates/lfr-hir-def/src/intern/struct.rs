use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Struct(salsa::InternId);

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct StructData {
    pub name: Arc<String>,
    pub fields: Arc<Vec<Field>>,
}

impl_intern!(Struct, StructData, intern_struct, lookup_intern_struct);
