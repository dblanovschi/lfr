use lfr_base_db::salsa;

use crate::intern::{Field, FieldData, Struct, StructData, TypeRef, TypeRefData};

#[salsa::query_group(HirDefStorage)]
pub trait DefDatabase: InternDatabase {
}

#[salsa::query_group(InternDatabaseStorage)]
pub trait InternDatabase: lfr_vfs::VfsDatabase {
    #[salsa::interned]
    fn intern_struct(&self, data: StructData) -> Struct;
    #[salsa::interned]
    fn intern_field(&self, data: FieldData) -> Field;
    #[salsa::interned]
    fn intern_type_ref(&self, data: TypeRefData) -> TypeRef;
}