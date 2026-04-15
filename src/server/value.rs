use aws_sdk_s3::Client;
use axum::{body::Bytes, http::StatusCode, response::IntoResponse};
use sqlx::{prelude::FromRow, Pool, Postgres};
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::job_upload_service::api::Task;

pub enum JobCreationError {
    Failed,
    AlreadyExists,
    DBError(String)
}

pub enum FileUploadError {
    JobCreationError(JobCreationError),
    S3UploadFailed(String),
    JobQueueFailed,
    NoFileUploaded,
    EnqueueFailed,
    ApiFailure(String)
}

pub  struct FileObject {
    pub file_key: String,
    pub file_data: Bytes
}


pub struct RowData {
    pub status: Option<String>,
    pub total_pages: Option<i32>,
    pub completed_pages: Option<i32>,
}

#[derive(Debug, FromRow)]
pub struct RowDataResult {
    pub id: Uuid,
    pub status: String,
}

impl From<JobCreationError> for FileUploadError {
    fn from(err: JobCreationError) -> Self {
        FileUploadError::JobCreationError(err)
    }
}

impl IntoResponse for FileUploadError {
    fn into_response(self) -> axum::response::Response {
        let body = match self {
            Self::S3UploadFailed(e) => e,
            Self::JobQueueFailed => "Job could not be queue in Job queue".to_string(),
            Self::JobCreationError(JobCreationError::AlreadyExists) => "Job cannot be created, it already exists".to_string(),
            Self::JobCreationError(JobCreationError::Failed) => "Job creation failed".to_string(),
            Self::JobCreationError(JobCreationError::DBError(e)) => e,
            Self::NoFileUploaded => "File not uploaded".to_string(),
            Self::EnqueueFailed => "Task could not be enqueue".to_string(),
            Self::ApiFailure(e) => e.to_string()

        };
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[derive(Clone)]
pub struct AppState{
    pub db_conn: Pool<Postgres>,
    pub s3_client: Client,
    pub job_sender: Sender<Task>
}
