use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Info {
    pub invites: bool,
    pub production: bool,
    pub version: String,
}
