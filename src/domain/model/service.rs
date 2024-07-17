use serde::{Deserialize, Serialize};

use super::{ContactPoint, Tag};

#[derive(Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub summary: String,
    pub description: String,
    pub tags: Vec<Tag>,
    pub contact_points: Vec<ContactPoint>,
}
