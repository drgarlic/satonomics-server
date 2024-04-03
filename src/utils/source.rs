use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Source {
    pub name: String,
    pub url: String,
    pub color: String,
}
