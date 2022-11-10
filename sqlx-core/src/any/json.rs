use crate::types::Json;

#[cfg(all(
    any(feature = "mysql", feature = "sqlite", feature = "postgres"),
    not(feature = "mssql")
))]
impl<T> crate::types::Type<crate::any::Any> for Json<T> {
    fn type_info() -> crate::any::AnyTypeInfo {
        // FIXME: nicer panic explaining why this isn't possible
        unimplemented!()
    }

    fn compatible(ty: &crate::any::AnyTypeInfo) -> bool {
        match &ty.0 {
            #[cfg(feature = "postgres")]
            crate::any::type_info::AnyTypeInfoKind::Postgres(ty) => {
                <Json<T> as crate::types::Type<crate::postgres::Postgres>>::compatible(&ty)
            }

            #[cfg(feature = "mysql")]
            crate::any::type_info::AnyTypeInfoKind::MySql(ty) => {
                <Json<T> as crate::types::Type<crate::mysql::MySql>>::compatible(&ty)
            }

            #[cfg(feature = "sqlite")]
            crate::any::type_info::AnyTypeInfoKind::Sqlite(ty) => {
                <Json<T> as crate::types::Type<crate::sqlite::Sqlite>>::compatible(&ty)
            }

            #[cfg(feature = "mssql")]
            crate::any::type_info::AnyTypeInfoKind::Mssql(ty) => {
                <Json<T> as crate::types::Type<crate::mssql::Mssql>>::compatible(&ty)
            }
        }
    }
}

#[cfg(all(
    any(feature = "mysql", feature = "sqlite", feature = "postgres"),
    not(feature = "mssql")
))]
impl<'q, T> crate::encode::Encode<'q, crate::any::Any> for Json<T>
where
    Json<T>: crate::any::AnyEncode<'q>,
{
    fn encode_by_ref(&self, buf: &mut crate::any::AnyArgumentBuffer<'q>) -> crate::encode::IsNull {
        match &mut buf.0 {
            #[cfg(feature = "postgres")]
            crate::any::arguments::AnyArgumentBufferKind::Postgres(args, _) => args.add(self),

            #[cfg(feature = "mysql")]
            crate::any::arguments::AnyArgumentBufferKind::MySql(args, _) => args.add(self),

            #[cfg(feature = "mssql")]
            crate::any::arguments::AnyArgumentBufferKind::Mssql(args, _) => args.add(self),

            #[cfg(feature = "sqlite")]
            crate::any::arguments::AnyArgumentBufferKind::Sqlite(args) => args.add(self),
        }

        // unused
        crate::encode::IsNull::No
    }
}

#[cfg(all(
    any(feature = "mysql", feature = "sqlite", feature = "postgres"),
    not(feature = "mssql")
))]
impl<'r, T> crate::decode::Decode<'r, crate::any::Any> for Json<T>
where
    Json<T>: crate::any::AnyDecode<'r>,
{
    fn decode(value: crate::any::AnyValueRef<'r>) -> Result<Self, crate::error::BoxDynError> {
        match value.kind {
            #[cfg(feature = "mysql")]
            crate::any::value::AnyValueRefKind::MySql(value) => {
                <Json<T> as crate::decode::Decode<'r, crate::mysql::MySql>>::decode(value)
            }

            #[cfg(feature = "sqlite")]
            crate::any::value::AnyValueRefKind::Sqlite(value) => {
                <Json<T> as crate::decode::Decode<'r, crate::sqlite::Sqlite>>::decode(value)
            }

            #[cfg(feature = "mssql")]
            crate::any::value::AnyValueRefKind::Mssql(value) => {
                <Json<T> as crate::decode::Decode<'r, crate::mssql::Mssql>>::decode(value)
            }

            #[cfg(feature = "postgres")]
            crate::any::value::AnyValueRefKind::Postgres(value) => {
                <Json<T> as crate::decode::Decode<'r, crate::postgres::Postgres>>::decode(value)
            }
        }
    }
}
