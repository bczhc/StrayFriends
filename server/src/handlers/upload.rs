use crate::handlers::handle_errors;
use crate::jwt::validate_token;
use crate::{api_ok, include_sql, mutex_lock, ApiExtension, AuthHeader, CONFIG};
use anyhow::anyhow;
use axum::body::Body;
use axum::extract::Multipart;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::{debug_handler, extract};
use axum_extra::headers::{ContentLength, ContentType};
use axum_extra::TypedHeader;
use image::{ImageFormat, ImageReader};
use log::info;
use once_cell::sync::Lazy;
use sha1::Sha1;
use std::fs;
use std::fs::File;
use std::io::{Cursor, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use tokio_util::io::ReaderStream;

static FILE_DIR: Lazy<PathBuf> = Lazy::new(|| {
    let guard = mutex_lock!(CONFIG);
    let path = guard.upload_path.clone();
    path.try_creation_dir()
});

fn stored_file_path(hex: &str) -> PathBuf {
    let prefix = &hex[..2];
    FILE_DIR.join(prefix).try_creation_dir().join(hex)
}

#[debug_handler]
pub async fn upload_image(
    ext: ApiExtension,
    auth: AuthHeader,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let result: anyhow::Result<_> = try {
        // only allow logged user to upload images
        let _ = validate_token!(auth);
        let first = multipart
            .next_field()
            .await?
            .ok_or_else(|| anyhow!("Empty part one"))?;
        let bytes = &first.bytes().await?;

        let mut hasher = Sha1::default();
        let mut buf = Cursor::new(Vec::new());
        let mut duplicator =
            bczhc_lib::io::duplicator::StreamDuplicator::new(&mut hasher, &mut buf);

        // TODO: chunked-write to improve performance
        for &x in bytes {
            duplicator.write_all(&[x])?;
        }
        duplicator.flush()?;

        let digest = digest::FixedOutput::finalize_fixed(hasher);
        let digest_hex = hex::encode(digest.as_slice());
        let stored_file = stored_file_path(&digest_hex);

        buf.seek(SeekFrom::Start(0))?;
        let image = ImageReader::new(&mut buf)
            .with_guessed_format()?
            .decode()?
            // remove the alpha channel due to the jpeg output
            .to_rgb8();
        image.save_with_format(&stored_file, ImageFormat::Jpeg)?;

        let db = &ext.db;

        let (record,): (i64,) = sqlx::query_as("select count(*) from image where id = $1")
            .bind(&digest_hex)
            .fetch_one(db)
            .await?;
        if record == 0 {
            sqlx::query(include_sql!("insert-image-id"))
                .bind(&digest_hex)
                .execute(db)
                .await?;
        }

        info!("Uploaded file: {}", stored_file.display());

        return api_ok!(digest_hex);
    };
    handle_errors!(result)
}

#[debug_handler]
pub async fn image(extract::Path(path): extract::Path<(String,)>) -> impl IntoResponse {
    let result: anyhow::Result<_> = try {
        let path = stored_file_path(&path.0);
        let file = match tokio::fs::File::open(&path).await {
            Ok(file) => file,
            Err(_) => return StatusCode::NOT_FOUND.into_response(),
        };
        let file_size = path.metadata()?.len();
        let stream = ReaderStream::new(file);
        let headers = (
            TypedHeader(ContentType::jpeg()),
            TypedHeader(ContentLength(file_size)),
        );
        return (headers, Body::from_stream(stream)).into_response();
    };
    handle_errors!(result)
}

pub trait PathExt {
    fn try_creation_dir(self) -> Self;

    fn try_creation_file(self) -> Self;
}

impl<P> PathExt for P
where
    P: AsRef<Path>,
{
    fn try_creation_dir(self) -> Self {
        let path = self.as_ref();
        if !path.exists() {
            // PANIC: treat IO errors fatal
            fs::create_dir(path)
                .unwrap_or_else(|_| panic!("Dir creation failed: {}", path.display()));
        }
        self
    }

    fn try_creation_file(self) -> Self {
        let path = self.as_ref();
        if !path.exists() {
            // PANIC: treat IO errors fatal
            File::create(path).expect("File creation failed");
        }
        self
    }
}
