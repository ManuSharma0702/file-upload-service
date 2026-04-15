use reqwest::Client;
use serde::Serialize;
use tokio::sync::mpsc::{self, Receiver, Sender};


//Split Task
#[derive(Debug, Serialize)]
pub struct Task {
    pub task_type: String,
    pub job_id:  String,
    pub file_url: String,
    pub retry_left: u32
}

pub struct UploadServicePayload {
    pub task: Task,
}

#[derive(Debug)]
pub enum UploadError {
    NotFound,
    UploadFailed(String)
}

pub struct UploadService {
    upload_service_tx: Sender<Task>,
    upload_service_rx: Receiver<Task>
}

impl UploadService {

    pub async fn execute(&mut self) {
        while let Some(task) = self.upload_service_rx.recv().await {
            if let Err(err) = self.upload(task).await {
                eprintln!("Upload failed: {:?}", err);
            }
        }
    }

    async fn upload(&self, payload: Task) -> Result<(), UploadError> {
        let client  = Client::new();

        let url = "http://127.0.0.1:8080/push";

        //TODO:On error update the status of the job in db to split_enqueue_failed.
        //Keep a background worker which loops and checks for enqueue_failed jobs  and retry them.
        //Add a new column to DB called enqueue_attempt_left. On each queue failure decrement the
        //count. If count zero then instead of split_enqueue_failed update status to dead.
        let _ = client
            .post(url)
            .json(&payload)
            .send()
            .await
            .map_err(|e| UploadError::UploadFailed(e.to_string()))?;

        Ok(())
    }

    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(1024);
        Self { upload_service_tx: sender, upload_service_rx: receiver }
    }

    pub fn get_sender(&self) -> Sender<Task> {
        self.upload_service_tx.clone()
    }

}

impl Default for UploadService {
    fn default() -> Self {
        Self::new()
    }
}
