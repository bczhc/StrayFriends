use sqlx::FromRow;

#[derive(FromRow)]
pub struct Password {
    pub blake3: String,
    pub salt: String,
}

impl Password {
    pub fn validate(&self, plain_password: &str) -> bool {
        let data = format!("{plain_password}{}", self.salt);
        let hash = blake3::hash(data.as_bytes());
        hex::encode(hash.as_bytes()) == self.blake3
    }
}
