use std::error::Error;

use postgres::types::{IsNull, ToSql, Type, private::BytesMut, to_sql_checked};

use crate::data_types::Value;

impl ToSql for Value {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match self {
            Value::Null => Ok(IsNull::Yes),
            Value::Bool(v) => v.to_sql(ty, out),
            Value::Integer(v) => match *ty {
                Type::INT2 => i16::try_from(*v)?.to_sql(ty, out),
                Type::INT4 => i32::try_from(*v)?.to_sql(ty, out),
                Type::INT8 => v.to_sql(ty, out),
                _ => v.to_sql(ty, out),
            },
            Value::Float(v) => v.to_sql(ty, out),
            Value::Text(v) => v.to_sql(ty, out),
        }
    }

    fn accepts(ty: &Type) -> bool {
        <bool as ToSql>::accepts(ty)
            || <i16 as ToSql>::accepts(ty)
            || <i32 as ToSql>::accepts(ty)
            || <i64 as ToSql>::accepts(ty)
            || <f64 as ToSql>::accepts(ty)
            || <String as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}
