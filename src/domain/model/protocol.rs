use serde::{Deserialize, Serialize};

/// `Protocol` represents a communication (transport or application) protocol
/// that a `Service` is capable of talking with the outside world, through a `ContactPoint`.
#[derive(Debug, Deserialize, Serialize)]
pub struct Protocol {
    pub name: String,
    pub description: String,
    pub required_props: Vec<String>,
    pub optional_props: Vec<String>,
}
