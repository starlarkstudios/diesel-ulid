use std::{ops::Deref, io::Write, str::FromStr, fmt::Debug};

use diesel::{deserialize::{FromSql, self}, sql_types::Uuid, pg::{Pg, PgValue}, serialize::{ToSql, Output, self, IsNull}, FromSqlRow};
use diesel::expression::AsExpression;
use rusty_ulid::{DecodingError, Ulid};
use serde::Serialize;
use serde::Deserialize;



#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Hash)]
#[derive(AsExpression, FromSqlRow)]
#[diesel(sql_type = Uuid)]
pub struct ArunaUlid(rusty_ulid::Ulid);


impl TryFrom<&[u8]> for ArunaUlid {
    type Error = DecodingError;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        Ok(ArunaUlid{0: rusty_ulid::Ulid::try_from(value)?})
    }
}

impl ArunaUlid {
    pub fn generate() -> Self {
        ArunaUlid(Ulid::generate())
    }
}

impl Debug for ArunaUlid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.0.to_string())
    }
}

impl Deref for ArunaUlid {
    type Target = rusty_ulid::Ulid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromStr for ArunaUlid {
    type Err = DecodingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {0: Ulid::from_str(s)? })
    }
}



impl FromSql<Uuid, Pg> for ArunaUlid {
    fn from_sql(value: PgValue<'_>) -> deserialize::Result<Self> {
        ArunaUlid::try_from(value.as_bytes()).map_err(Into::into)
    }
}

impl ToSql<Uuid, Pg> for ArunaUlid {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
        out.write_all(<[u8; 16]>::from(self.0).as_slice())
            .map(|_| IsNull::No)
            .map_err(Into::into)
    }
}