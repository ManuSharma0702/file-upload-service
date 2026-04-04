use std::env;
use std::error::Error;
use aws_sdk_s3::primitives::ByteStream;
use dotenvy::dotenv;

use axum::extract::{Multipart, State};
use axum::http::StatusCode;
use axum::Router;
use axum::routing::{get, post};
use sqlx::postgres::PgPoolOptions;

use crate::server::db::create_job;
use crate::server::s3::upload_to_s3;
use crate::server::value::{AppState, FileObject, FileUploadError, JobCreationError};
use aws_config::load_from_env;
use aws_sdk_s3::Client;

//On init, I need a db connection for job creation
//An s3 connection for upload
//A job queue connection. These connection will be shared
pub async fn run() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = PgPoolOptions::new()
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    sqlx::migrate!().run(&db).await.expect("Migrations failed");

    let config = load_from_env().await;
    let client = Client::new(&config);

    let state = AppState {
        db_conn: db,
        s3_client: client
    };

    let app = Router::new()
        .route("/", get(handle_home))
        .route("/upload", post(handle_file_upload))
        .with_state(state);
        
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await?;
    println!("Server running on 127.0.0.1:8000...");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn handle_home() -> &'static str {
    "Hello API"
}

//Endpoint for uploading PDFs only
//Creates a job in DB, initialising the process
//Uploads the PDF to s3
//Push splitting task to job queue
async fn handle_file_upload(
    State(state): State<AppState>,
    multipart: Multipart
) -> Result<StatusCode, FileUploadError> {
    let file = accept_form(multipart)
        .await
        .ok_or(FileUploadError::NoFileUploaded)?;
    let job_id = create_job(&state.db_conn).await?; 
    upload_to_s3(&state.s3_client, file, "fileocr").await?;
    Ok(StatusCode::OK)
}

async fn accept_form(mut multipart: Multipart) -> Option<FileObject> {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name()?.to_string();
        let data = field.bytes().await.unwrap();
        return Some(FileObject {
            file_key: file_name,
            file_data: data,
        });
    }
    None
}
