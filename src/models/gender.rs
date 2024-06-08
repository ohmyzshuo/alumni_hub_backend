use diesel::pg::{Pg, PgValue};
use diesel::serialize::{ToSql, Output, IsNull};
use diesel::deserialize::{FromSql, Result as DeserializeResult, FromSqlRow};
use diesel::sql_types::Text;
use std::io::Write;
use serde::{Deserialize, Serialize};
use diesel::expression::AsExpression;
use diesel::internal::derives::as_expression::Bound;
use diesel::sql_types::Nullable;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, FromSqlRow)]
pub enum Gender {
    #[serde(rename = "NotSpecified")]
    NotSpecified,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "Female")]
    Female,
    #[serde(rename = "Male")]
    Male,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::NotSpecified
    }
}

impl FromSql<Text, Pg> for Gender {
    fn from_sql(value: PgValue<'_>) -> DeserializeResult<Self> {
        match value.as_bytes() {
            b"NotSpecified" => Ok(Gender::NotSpecified),
            b"Other" => Ok(Gender::Other),
            b"Female" => Ok(Gender::Female),
            b"Male" => Ok(Gender::Male),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

impl ToSql<Text, Pg> for Gender {
    fn to_sql(&self, out: &mut Output<Pg>) -> diesel::serialize::Result {
        match *self {
            Gender::NotSpecified => out.write_all(b"NotSpecified")?,
            Gender::Other => out.write_all(b"Other")?,
            Gender::Female => out.write_all(b"Female")?,
            Gender::Male => out.write_all(b"Male")?,
        }
        Ok(IsNull::No)
    }
}
impl AsExpression<Nullable<Text>> for Gender {
    type Expression = Bound<Nullable<Text>, Self>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}

impl<'a> AsExpression<Nullable<Text>> for &'a Gender {
    type Expression = Bound<Nullable<Text>, &'a Gender>;

    fn as_expression(self) -> Self::Expression {
        Bound::new(self)
    }
}