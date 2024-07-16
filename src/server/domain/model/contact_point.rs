use serde::{Deserialize, Serialize};

use super::Protocol;

#[derive(Serialize, Deserialize)]
pub struct ContactPoint {
    pub name: String,
    pub protocol: Box<dyn Protocol>,
}
