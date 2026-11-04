use std::error::Error;

use hyperion_core::value::Value;
use postgres::types::{FromSql, IsNull, ToSql, Type, private::BytesMut, to_sql_checked};
use thiserror::Error;

use crate::value_types::SQLValue;

impl ToSql for SQLValue {
    fn to_sql(
        &self,
        ty: &Type,
        out: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        match &self.0 {
            Value::Null => Ok(IsNull::Yes),
            Value::Boolean(v) => v.to_sql(ty, out),
            Value::Integer(v) => match *ty {
                Type::INT2 => i16::try_from(v.as_i64())?.to_sql(ty, out),
                Type::INT4 => i32::try_from(v.as_i64())?.to_sql(ty, out),
                Type::INT8 => v.as_i64().to_sql(ty, out),
                _ => v.as_i64().to_sql(ty, out),
            },
            Value::UInteger(v) => match *ty {
                Type::INT4 => u32::try_from(v.as_u64())?.to_sql(ty, out),
                _ => todo!(),
            },
            Value::Float(v) => v.as_f64().to_sql(ty, out),
            Value::String(v) => v.to_sql(ty, out),
            Value::Bytes(v) => v.as_array().to_sql(ty, out),
            Value::Timestamp(v) => v.as_i64().to_sql(ty, out),
            Value::Array(array) => todo!(),
            Value::Map(map) => todo!(),
        }
    }

    fn accepts(ty: &Type) -> bool {
        <bool as ToSql>::accepts(ty)
            || <u32 as ToSql>::accepts(ty)
            || <i32 as ToSql>::accepts(ty)
            || <i64 as ToSql>::accepts(ty)
            || <f64 as ToSql>::accepts(ty)
            || <String as ToSql>::accepts(ty)
            || <Vec<u8> as ToSql>::accepts(ty)
    }

    to_sql_checked!();
}

#[derive(Debug, Error)]
pub enum SQLError {
    #[error("Invalid column: {0}")]
    InvalidColumn(String),
    #[error("Invalid table: {0}")]
    InvalidTable(String),
    #[error("{0}")]
    Custom(String),
}

impl<'a> FromSql<'a> for SQLValue {
    fn from_sql(
        ty: &postgres::types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let value: Value = match *ty {
            Type::BOOL => {
                let v: bool = bool::from_sql(ty, raw)?;
                Value::from(v)
            }
            Type::FLOAT4 | Type::FLOAT8 => {
                let v: f64 = f64::from_sql(ty, raw)?;
                Value::from(v)
            }
            Type::INT4 => {
                let v: i32 = i32::from_sql(ty, raw)?;
                Value::from(v as i64)
            }
            Type::INT2 | Type::INT8 => {
                let v: i64 = i64::from_sql(ty, raw)?;
                Value::from(v)
            }
            Type::TEXT | Type::VARCHAR => {
                let v: String = String::from_sql(ty, raw)?;
                Value::from(v)
            }
            Type::BYTEA => {
                let v: Vec<u8> = raw.into();
                Value::from(v)
            }
            _ => return Err(SQLError::Custom("".to_string()).into()),
        };

        Ok(SQLValue(value))
    }

    fn accepts(ty: &postgres::types::Type) -> bool {
        <bool as ToSql>::accepts(ty)
            || <i16 as ToSql>::accepts(ty)
            || <i32 as ToSql>::accepts(ty)
            || <i64 as ToSql>::accepts(ty)
            || <f64 as ToSql>::accepts(ty)
            || <String as ToSql>::accepts(ty)
            || <Vec<u8> as ToSql>::accepts(ty)
    }
}
