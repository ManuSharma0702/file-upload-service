use sqlx::{Pool, Postgres};

use crate::server::value::{JobCreationError, RowData, RowDataResult};

pub async fn create_job(db_conn: &Pool<Postgres>) -> Result<String, JobCreationError>{
    let row_data = RowData {
        status: "PENDING".to_string(),
        total_pages: 0,
        completed_pages: 0
    };
    insert_job(db_conn, row_data).await
}

async fn insert_job(
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
    .bind(row_data.status)
    .bind(row_data.completed_pages)
    .bind(row_data.total_pages)
    .fetch_one(db_conn)
    .await
    .map_err(|e| JobCreationError::DBError(e.to_string()))?; // fixed below

    Ok(data.id.to_string())
}
