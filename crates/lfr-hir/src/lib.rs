use std::ops::Deref;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BytePos(pub u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span
{
    pub lo: BytePos,
    pub hi: BytePos,
}

macro_rules! id_ty {
    ($name:ident) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(usize);

        impl From<usize> for $name
        {
            fn from(id: usize) -> Self { Self(id) }
        }

        impl From<$name> for usize
        {
            fn from(id: $name) -> Self { id.0 }
        }
    };
}

id_ty!(FileId);

#[derive(Clone, Debug)]
pub struct HirName<'db>
{
    pub name: &'db str,
    pub span: Span,
}

impl<'db> Deref for HirName<'db>
{
    type Target = str;

    fn deref(&self) -> &Self::Target { self.name }
}

impl<'db> PartialEq for HirName<'db>
{
    fn eq(&self, other: &Self) -> bool { self.name.eq(other.name) }
}

impl<'db> PartialEq<String> for HirName<'db>
{
    fn eq(&self, other: &String) -> bool { self.name.eq(other) }
}

pub struct HirStructDecl<'db>
{
    pub name:   HirName<'db>,
    pub fields: Vec<HirStructField<'db>>,
}

pub struct HirStructFieldList<'db>
{
    pub fields: Vec<HirStructField<'db>>,
}

pub struct HirStructField<'db>
{
    pub name: HirName<'db>,
    pub ty:   HirTypeRef<'db>,
}

pub struct HirEnumDecl<'db>
{
    pub name:     HirName<'db>,
    pub variants: Vec<HirEnumVariant<'db>>,
}

pub struct HirEnumVariant<'db>
{
    pub name: HirName<'db>,
}

pub struct HirFnDecl<'db>
{
    pub name: HirName<'db>,
    pub args: Vec<HirFnArg<'db>>,
    pub ret:  HirTypeRef<'db>,
}

pub struct HirFnArg<'db>
{
    pub name: HirName<'db>,
    pub ty:   HirTypeRef<'db>,
}

pub enum HirDecl<'db>
{
    Struct(HirStructDecl<'db>),
    Enum(HirEnumDecl<'db>),
    Fn(HirFnDecl<'db>),
}

pub struct HirTypeRef<'db>
{
    pub ty: HirName<'db>,
}
