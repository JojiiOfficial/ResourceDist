use serde::{Deserialize, Serialize};

/// Error response body
#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl ErrorResponse {
    #[inline]
    pub fn new(error: String, message: String) -> Self {
        Self { error, message }
    }
}
