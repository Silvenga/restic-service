use crate::jobs::JobManager;
use std::sync::Arc;

pub struct ApiState {
    pub job_manager: Arc<JobManager>,
}
