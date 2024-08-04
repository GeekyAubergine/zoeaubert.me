use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateSillyNamesRequest {
    pub silly_names: Vec<String>,
}

impl UpdateSillyNamesRequest {
    pub fn new(silly_names: Vec<String>) -> Self {
        Self { silly_names }
    }
}
