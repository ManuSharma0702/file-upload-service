use aws_sdk_s3::{primitives::ByteStream, Client};

use crate::server::value::{FileObject, FileUploadError};

pub async fn upload_to_s3(s3_client: &Client , file: FileObject, bucket_name: &str) -> Result<(), FileUploadError>{
    s3_client
        .put_object()
        .bucket(bucket_name)
        .key(file.file_key)
        .body(ByteStream::from(file.file_data))
        .send()
        .await
        .map_err(|e| FileUploadError::S3UploadFailed(e.to_string()))?;
    println!("Uploaded successfully");
    Ok(())
}

