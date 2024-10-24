use crate::{include_sql, random_string};
use serde::{Deserialize, Serialize};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::PgQueryResult;
use sqlx::{Database, Decode, Encode, Error, FromRow, PgPool, Postgres, Type};
use std::ops::Deref;
use strum::AsRefStr;

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

pub type ImageId = String;

#[derive(FromRow, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: Uid,
    name: String,
    email: String,
    avatar_image_id: Option<ImageId>,
    bio: Option<String>,
    #[serde(skip_serializing)]
    password: Password,
    gender_type: GenderTypePg,
    gender_other: String,
}

#[derive(Serialize, Deserialize, Debug, AsRefStr)]
pub enum Gender {
    #[strum(serialize = "male")]
    Male,
    #[strum(serialize = "female")]
    Female,
    #[strum(serialize = "secret")]
    Secret,
    #[strum(serialize = "other")]
    Other(String),
}

impl Gender {
    pub fn from(r#type: &str, other: Option<impl Into<String>>) -> Option<Self> {
        match r#type {
            "male" => Some(Self::Male),
            "female" => Some(Self::Female),
            "secret" => Some(Self::Secret),
            "other" => Some(Self::Other(other?.into())),
            _ => None,
        }
    }
}

#[derive(Type, FromRow, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "Gender")]
pub struct GenderPg {
    pub r#type: String,
    pub other: String,
}

#[derive(Type, Debug, Serialize, Deserialize)]
#[sqlx(type_name = "GenderType", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
/// TODO: reduce redundant Pg type definitions
pub enum GenderTypePg {
    Male,
    Female,
    Secret,
    Other,
}

impl GenderPg {
    fn new(r#type: String, other: String) -> Self {
        Self { r#type, other }
    }
}

impl From<Gender> for GenderPg {
    fn from(value: Gender) -> Self {
        let type_string = String::from(value.as_ref());
        match value {
            Gender::Male => Self::new(type_string, Default::default()),
            Gender::Female => Self::new(type_string, Default::default()),
            Gender::Secret => Self::new(type_string, Default::default()),
            Gender::Other(x) => Self::new(type_string, x),
        }
    }
}

pub async fn change_password(db: &PgPool, new: &Password) -> Result<PgQueryResult, Error> {
    sqlx::query(include_sql!("update-user-info"))
        .bind(&new.blake3)
        .bind(&new.salt)
        .execute(db)
        .await
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnimalPostForm {
    name: String,
    description: String,
    content: String,
    image_id_list: Vec<String>,
    mobile_number: String,
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

#[cfg(test)]
mod test {
    use crate::db::Password;
    use sqlx::Type;

    #[test]
    pub fn test() {
        println!("{}", Password::type_info());
        println!("{:?}", Password::type_info());
    }
}
