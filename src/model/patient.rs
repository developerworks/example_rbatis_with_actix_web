use lombok::Setter;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Clone, Deserialize, Serialize, Setter)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct User {
    pub id: Option<u64>,
    pub name: Option<String>,
}
