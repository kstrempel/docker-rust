use serde_json;

#[derive(Debug)]
pub enum DockerError {
    Docker(String),
    Json(serde_json::error::Error)
}

