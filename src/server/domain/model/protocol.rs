use serde::{Deserialize, Serialize};

/// A protocol represents a communication (transport or application) protocol
/// that a `Service` is capable to talk with the outside world.
#[typetag::serde(tag = "protocol")]
pub trait Protocol {
    /// Get the name of the protocol.
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn mandatory_props(&self) -> Vec<String>;
}

#[derive(Deserialize, Serialize)]
pub struct HTTP {
    pub port: u16,
}

#[typetag::serde]
impl Protocol for HTTP {
    fn name(&self) -> String {
        String::from("HTTP")
    }
    fn description(&self) -> String {
        String::from("HTTP protocol")
    }
    fn mandatory_props(&self) -> Vec<String> {
        vec!["port".to_string()]
    }
}
