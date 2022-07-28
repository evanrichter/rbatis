use std::borrow::Cow;
use crate::arguments::{PgArgumentBuffer, PgArguments};
use crate::type_info::PgTypeInfo;
use crate::types::Oid;
use rbs::Value;
use std::mem;
use std::str::FromStr;
use rbdc::date::Date;
use rbdc::datetime::DateTime;
use rbdc::decimal::Decimal;
use rbdc::Error;
use rbdc::json::Json;
use rbdc::timestamp::Timestamp;
use rbdc::types::time::Time;
use rbdc::uuid::Uuid;
use crate::types::byte::Bytea;
use crate::types::money::Money;
use crate::types::timestamptz::Timestamptz;
use crate::types::timez::Timetz;

pub enum IsNull {
    No,
    Yes,
}

pub trait TypeInfo {
    fn type_info(&self) -> PgTypeInfo;
}

pub trait Encode {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error>;
}

impl From<Vec<Value>> for PgArguments {
    fn from(args: Vec<Value>) -> Self {
        let mut arg = PgArguments {
            types: Vec::with_capacity(args.len()),
            buffer: PgArgumentBuffer::default(),
        };
        for x in args {
            arg.add(x).unwrap();
        }
        arg
    }
}

impl TypeInfo for Value {
    fn type_info(&self) -> PgTypeInfo {
        match self {
            Value::Null => PgTypeInfo::UNKNOWN,
            Value::Bool(_) => {
                PgTypeInfo::BOOL
            }
            Value::I32(_) => {
                PgTypeInfo::INT4
            }
            Value::I64(_) => {
                PgTypeInfo::INT8
            }
            Value::U32(_) => {
                PgTypeInfo::INT4
            }
            Value::U64(_) => {
                PgTypeInfo::INT8
            }
            Value::F32(_) => {
                PgTypeInfo::FLOAT4
            }
            Value::F64(_) => {
                PgTypeInfo::FLOAT8
            }
            Value::String(_) => PgTypeInfo::VARCHAR,
            Value::Binary(_) => {
                PgTypeInfo::BYTEA_ARRAY
            }
            Value::Array(arr) => {
                if arr.len() == 0 {
                    return PgTypeInfo::UNKNOWN;
                }
                arr[0].type_info().clone().to_array_element().unwrap_or(PgTypeInfo::UNKNOWN)
            }
            Value::Map(_) => {
                PgTypeInfo::UNKNOWN
            }
            Value::Ext(type_name, _) => {
                match *type_name {
                    "Uuid" => {
                        PgTypeInfo::UUID
                    }
                    //decimal = 12345678
                    "Decimal" => {
                        PgTypeInfo::NUMERIC
                    }
                    //Date = "1993-02-06"
                    "Date" => {
                        PgTypeInfo::DATE
                    }
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "Time" => {
                        PgTypeInfo::TIME
                    }
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "Timestamp" => {
                        PgTypeInfo::TIMESTAMP
                    }
                    "DateTime" => {
                        PgTypeInfo::TIMESTAMP
                    }
                    "Bool" => {
                        PgTypeInfo::BOOL
                    }
                    "Bytea" => {
                        PgTypeInfo::BYTEA
                    }
                    "Char" => {
                        PgTypeInfo::CHAR
                    }
                    "Name" => {
                        PgTypeInfo::NAME
                    }
                    "Int8" => {
                        PgTypeInfo::INT8
                    }
                    "Int2" => {
                        PgTypeInfo::INT2
                    }
                    "Int4" => {
                        PgTypeInfo::INT4
                    }
                    "Text" => {
                        PgTypeInfo::TEXT
                    }
                    "Oid" => PgTypeInfo::OID,
                    "Json" => PgTypeInfo::JSON,
                    "Point" => {
                        PgTypeInfo::POINT
                    }
                    "Lseg" => {
                        PgTypeInfo::LSEG
                    }
                    "Path" => {
                        PgTypeInfo::PATH
                    }
                    "Box" => {
                        PgTypeInfo::BOX
                    }
                    "Polygon" => {
                        PgTypeInfo::POLYGON
                    }
                    "Line" => {
                        PgTypeInfo::LINE
                    }
                    "Cidr" => {
                        PgTypeInfo::CIDR
                    }
                    "Float4" => {
                        PgTypeInfo::FLOAT4
                    }
                    "Float8" => {
                        PgTypeInfo::FLOAT8
                    }
                    "Unknown" => {
                        PgTypeInfo::UNKNOWN
                    }
                    "Circle" => {
                        PgTypeInfo::CIRCLE
                    }
                    "Macaddr8" => {
                        PgTypeInfo::MACADDR8
                    }
                    "Macaddr" => {
                        PgTypeInfo::MACADDR
                    }
                    "Inet" => {
                        PgTypeInfo::INET
                    }
                    "Bpchar" => {
                        PgTypeInfo::BPCHAR
                    }
                    "Varchar" => {
                        PgTypeInfo::VARCHAR
                    }
                    "Timestamptz" => {
                        PgTypeInfo::TIMESTAMPTZ
                    }
                    "Interval" => {
                        PgTypeInfo::INTERVAL
                    }
                    "Timetz" => {
                        PgTypeInfo::TIMETZ
                    }
                    "Bit" => {
                        PgTypeInfo::BIT
                    }
                    "Varbit" => {
                        PgTypeInfo::VARBIT
                    }
                    "Numeric" => {
                        PgTypeInfo::NUMERIC
                    }
                    "Record" => {
                        PgTypeInfo::RECORD
                    }
                    "Jsonb" => {
                        PgTypeInfo::JSONB
                    }
                    "Int4Range" => {
                        PgTypeInfo::INT4_RANGE
                    }
                    "NumRange" => {
                        PgTypeInfo::NUM_RANGE
                    }
                    "TsRange" => {
                        PgTypeInfo::TS_RANGE
                    }
                    "TstzRange" => {
                        PgTypeInfo::TSTZ_RANGE
                    }
                    "DateRange" => {
                        PgTypeInfo::DATE_RANGE
                    }
                    "Int8Range" => {
                        PgTypeInfo::INT8_RANGE
                    }
                    "Jsonpath" => {
                        PgTypeInfo::JSONPATH
                    }
                    "Money" => {
                        PgTypeInfo::MONEY
                    }
                    "Void" => {
                        PgTypeInfo::VOID
                    }
                    "Custom" => {
                        PgTypeInfo::UNKNOWN
                    }
                    "DeclareWithName" => {
                        PgTypeInfo::UNKNOWN
                    }
                    "DeclareWithOid" => {
                        PgTypeInfo::UNKNOWN
                    }
                    _ => PgTypeInfo::UNKNOWN,
                }
            }
        }
    }
}

impl Encode for Value {
    fn encode(self, buf: &mut PgArgumentBuffer) -> Result<IsNull,Error> {
        Ok(match self {
            Value::Null => IsNull::Yes,
            Value::Bool(v) => {
                v.encode(buf)?
            }
            Value::I32(v) => {
                v.encode(buf)?
            }
            Value::I64(v) => {
                v.encode(buf)?
            }
            Value::U32(v) => {
                v.encode(buf)?
            }
            Value::U64(v) => {
                v.encode(buf)?
            }
            Value::F32(v) => {
                v.encode(buf)?
            }
            Value::F64(v) => {
                v.encode(buf)?
            }
            Value::String(v) => {
                //default -> string
                v.encode(buf)?
            }
            Value::Binary(v) => {
                v.encode(buf)?
            }
            Value::Array(v) => {
                v.encode(buf)?
            }
            Value::Map(v) => {
                IsNull::Yes
            }
            Value::Ext(type_name, v) => {
                match type_name {
                    "Uuid" => {
                        Uuid(v.into_string().unwrap_or_default()).encode(buf)?
                    }
                    //decimal = 12345678
                    "Decimal" => {
                       Decimal(v.into_string().unwrap_or_default()).encode(buf)?
                    }
                    //Date = "1993-02-06"
                    "Date" => {
                        Date(fastdate::Date::from_str(&v.into_string().unwrap_or_default()).unwrap()).encode(buf)?
                    }
                    //RFC3339NanoTime = "15:04:05.999999999"
                    "Time" => {
                        Time(fastdate::Time::from_str(&v.into_string().unwrap_or_default()).unwrap()).encode(buf)?
                    }
                    //RFC3339 = "2006-01-02 15:04:05.999999"
                    "Timestamp" => {
                        Timestamp(v.as_u64().unwrap_or_default()).encode(buf)?
                    }
                    "DateTime" => {
                        DateTime(fastdate::DateTime::from_str(&v.into_string().unwrap_or_default()).unwrap()).encode(buf)?
                    }
                    "Bytea" => {
                        Bytea(v.as_u64().unwrap_or_default() as u8).encode(buf)?
                    }
                    "Char" => {
                        v.into_string().unwrap_or_default().encode(buf)?
                    }
                    "Name" => {
                        v.into_string().unwrap_or_default().encode(buf)?
                    }
                    "Int8" => {
                        (v.as_i64().unwrap_or_default() as i32).encode(buf)?
                    }
                    "Int2" => {
                        (v.as_i64().unwrap_or_default() as i8).encode(buf)?
                    }
                    "Int4" => {
                        (v.as_i64().unwrap_or_default() as i16).encode(buf)?
                    }
                    "Text" => {
                        v.into_string().unwrap_or_default().encode(buf)?
                    }
                    "Oid" => Oid::from(v.as_u64().unwrap_or_default() as u32).encode(buf)?,
                    "Json" => Json(v.into_string().unwrap_or_default()).encode(buf)?,
                    "Point" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Lseg" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Path" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Box" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Polygon" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Line" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Cidr" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Float4" => {
                        (v.as_f64().unwrap_or_default() as f32).encode(buf)?
                    }
                    "Float8" => {
                        v.as_f64().unwrap_or_default().encode(buf)?
                    }
                    "Unknown" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Circle" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Macaddr8" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Macaddr" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Inet" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Bpchar" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Varchar" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Timestamptz" => {
                       Timestamptz(v.as_u64().unwrap_or_default()).encode(buf)?
                    }
                    "Interval" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Timetz" => {
                        Timetz(fastdate::Time::from_str(&v.into_string().unwrap_or_default()).unwrap()).encode(buf)?
                    }
                    "Bit" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Varbit" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Numeric" => {
                        Decimal(v.into_string().unwrap_or_default()).encode(buf)?
                    }
                    "Record" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Jsonb" => {
                        Json(v.into_string().unwrap_or_default()).encode(buf)?
                    }
                    "Int4Range" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "NumRange" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "TsRange" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "TstzRange" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "DateRange" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Int8Range" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Jsonpath" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Money" => {
                        Money(v.as_i64().unwrap_or_default()).encode(buf)?
                    }
                    "Void" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "Custom" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "DeclareWithName" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    "DeclareWithOid" => {
                        v.into_bytes().unwrap_or_default().encode(buf)?
                    }
                    _ => IsNull::Yes,
                }
            }
        })
    }
}






