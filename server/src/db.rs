use crate::{include_sql, random_string};
use serde::{Deserialize, Serialize};
use sqlx::encode::IsNull;
use sqlx::error::BoxDynError;
use sqlx::postgres::PgQueryResult;
use sqlx::{Database, Decode, Encode, Error, FromRow, PgPool, Postgres, Type};
use std::ops::Deref;
use strum::AsRefStr;

const PASSWORD_SALT_LENGTH: usize = 16;

#[derive(FromRow, Debug, Type, Serialize, Deserialize, Default)]
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
    pub id: Uid,
    pub name: String,
    pub email: String,
    pub avatar_image_id: Option<ImageId>,
    pub bio: Option<String>,
    #[serde(skip_serializing, skip_deserializing)]
    password: Password,
    pub gender_type: GenderTypePg,
    pub gender_other: Option<String>,
    pub admin: bool,
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
    pub name: String,
    pub description: String,
    pub content: String,
    // TODO: use a more type-aware definition: Vec<String>
    //  name: 'image_id_list[]' from Axios
    pub image_id_list: String,
    pub mobile_number: String,
}

#[derive(Serialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AnimalInfoQueryRow {
    pub post_id: RowId,
    pub username: String,
    pub user_avatar_image_id: Option<String>,
    pub name: String,
    pub description: String,
    pub content: String,
    pub image_id_list: Vec<String>,
    pub creation_time: TimestampSec,
    pub adopted: bool,
    pub post_uid: RowId,
}

#[derive(Serialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AdoptionRequestQueryRow {
    pub request_id: RowId,
    pub post_uid: RowId,
    pub animal_post_id: RowId,
    pub request_details: String,
    pub mobile_number: String,
}

#[derive(Serialize, Debug, FromRow)]
#[serde(rename_all = "camelCase")]
pub struct SquarePostRow {
    pub id: RowId,
    pub post_uid: RowId,
    pub content: String,
    pub images: Vec<String>,
    pub creation_time: TimestampSec,
}

type TimestampSec = UInt<u64>;

/// Also integer types in pgsql are signed. This is a bridge type.
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

/// Implement all the conversion traits.
macro impl_uint($pg_ty:ty, $rs_ty:ty) {
    impl Decode<'_, Postgres> for UInt<$rs_ty> {
        fn decode(value: <Postgres as Database>::ValueRef<'_>) -> Result<Self, BoxDynError> {
            <$pg_ty as Decode<'_, Postgres>>::decode(value).map(|x| Self(x as $rs_ty))
        }
    }

    impl Encode<'_, Postgres> for UInt<$rs_ty> {
        fn encode_by_ref(
            &self,
            buf: &mut <Postgres as Database>::ArgumentBuffer<'_>,
        ) -> Result<IsNull, BoxDynError> {
            <$pg_ty as Encode<'_, Postgres>>::encode(self.0 as $pg_ty, buf)
        }
    }

    impl Type<Postgres> for UInt<$rs_ty> {
        fn type_info() -> <Postgres as Database>::TypeInfo {
            <$pg_ty as Type<Postgres>>::type_info()
        }
    }
}

impl_uint!(i32, u32);
impl_uint!(i64, u64);

/// Type `SERIAL` in PgSQL.
pub type RowId = i64;
pub type Uid = RowId;
pub type PgCount = i64;

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
