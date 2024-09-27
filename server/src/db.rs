use std::ops::Deref;
use crate::random_string;
use serde::{Deserialize, Serialize};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::{Database, Decode, Encode, FromRow, Postgres, Type};

const PASSWORD_SALT_LENGTH: usize = 16;

#[derive(FromRow, Debug, Type, Serialize, Deserialize)]
pub struct Password {
    pub blake3: String,
    pub salt: String,
}

impl Password {
    pub fn validate(&self, plain_password: &str) -> bool {
        let new = Self::from_plain(plain_password, &self.salt);
        self.blake3 == new.blake3
    }

    pub fn from_plain(plain_password: &str, salt: &str) -> Self {
        let data = format!("{plain_password}{salt}");
        let hash = blake3::hash(data.as_bytes());
        Self {
            salt: salt.into(),
            blake3: hex::encode(hash.as_bytes()),
        }
    }

    pub fn new(blake3: impl Into<String>, salt: impl Into<String>) -> Self {
        Self {
            blake3: blake3.into(),
            salt: salt.into(),
        }
    }

    pub fn generate(plain_password: &str) -> Password {
        let random_salt = random_string(PASSWORD_SALT_LENGTH);
        Self::from_plain(plain_password, &random_salt)
    }
}

#[derive(FromRow, Debug, Type, Serialize, Deserialize)]
pub struct Image {
    id: String,
    width: UInt<u32>,
    height: UInt<u32>,
}

#[derive(FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    id: Uid,
    name: String,
    email: String,
    avatar: Option<Image>,
    bio: Option<String>,
    #[serde(skip_serializing)]
    password: Password,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UInt<T>(T);

impl<T> From<T> for UInt<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for UInt<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Decode<'_, Postgres> for UInt<u32> {
    fn decode(value: <Postgres as Database>::ValueRef<'_>) -> Result<Self, BoxDynError> {
        <i32 as Decode<'_, Postgres>>::decode(value).map(|x| Self(x as u32))
    }
}

impl Encode<'_, Postgres> for UInt<u32> {
    fn encode_by_ref(
        &self,
        buf: &mut <Postgres as Database>::ArgumentBuffer<'_>,
    ) -> Result<IsNull, BoxDynError> {
        <i32 as Encode<'_, Postgres>>::encode(self.0 as i32, buf)
    }
}

impl Type<Postgres> for UInt<u32> {
    fn type_info() -> <Postgres as Database>::TypeInfo {
        <i32 as Type<Postgres>>::type_info()
    }
}

pub type Uid = i64;