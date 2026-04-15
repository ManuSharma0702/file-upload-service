use sqlx::{Pool, Postgres};
use uuid::Uuid;
use std::str::FromStr;

use crate::server::value::{JobCreationError, RowData, RowDataResult};

pub async fn create_job(db_conn: &Pool<Postgres>) -> Result<String, JobCreationError>{
    let row_data = RowData {
        status: Some("PENDING_ENQUEUE".to_string()),
        total_pages: Some(0),
        completed_pages: Some(0) 
    };
    insert_row(db_conn, row_data).await
}

pub async fn fail_job(db_conn: &Pool<Postgres>, job_id: &str) -> Result<(), JobCreationError>{
    let row_data = RowData {
        status: Some("FAILED".to_string()),
        total_pages: None,
        completed_pages: None
    };
    update_row(db_conn, row_data, job_id).await
}

async fn update_row(
    db_conn: &Pool<Postgres>,
    row_data: RowData,
    job_id: &str
) -> Result<(), JobCreationError> {

    let uuid = Uuid::from_str(job_id)
        .map_err(|e| JobCreationError::DBError(e.to_string()))?;

    let _ = sqlx::query_as::<_, RowDataResult>(
        r#"
        UPDATE jobs
        SET status = $1
        WHERE id = $2
        RETURNING id, status
        "#
    )
    .bind(row_data.status.unwrap_or("FAILED".to_string()))
    .bind(uuid)
    .fetch_one(db_conn)
    .await
    .map_err(|e| JobCreationError::DBError(e.to_string()))?;

    Ok(())
}

async fn insert_row(
    db_conn: &Pool<Postgres>,
    row_data: RowData
) -> Result<String, JobCreationError> {

    let data = sqlx::query_as::<_, RowDataResult>(
        r#"
        INSERT INTO jobs (status, page_completed, total_pages)
        VALUES ($1, $2, $3)
        RETURNING id, status
        "#
    )
    .bind(row_data.status.unwrap_or("PENDING".to_string()))
    .bind(row_data.completed_pages.unwrap_or(0))
    .bind(row_data.total_pages.unwrap_or(0))
    .fetch_one(db_conn)
    .await
    .map_err(|e| JobCreationError::DBError(e.to_string()))?; // fixed below

    Ok(data.id.to_string())
}
