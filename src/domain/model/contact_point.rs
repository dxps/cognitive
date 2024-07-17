use serde::{Deserialize, Serialize};

use super::Protocol;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContactPoint {
    pub name: String,
    pub protocol: Protocol,
}
