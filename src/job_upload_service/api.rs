use tokio::sync::mpsc::{Receiver, Sender};

//Split Task
struct Task {
    task_type: String,
    job_id:  String,
    file_url: String,
    retry_left: u32
}

pub struct UploadServicePayload {
    pub task: Task,
}

pub enum UploadError {
    NotFound,
    UploadFailed
}

pub struct UploadService {
    upload_service_tx: Sender<Task>,
    upload_service_rx: Receiver<Task>
}

impl UploadService {

    async fn execute(&mut self) {
        while let Some(task) = self.upload_service_rx.recv().await {
            let res = self.upload(task);
        }
    }

    fn upload(&self, payload: Task) -> Result<(), UploadError> {
        Ok(())
    }
}
