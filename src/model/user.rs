use lombok::Setter;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use utoipa::ToSchema;

#[derive(Debug, Clone, Deserialize, Serialize, Setter, ToSchema)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct User {
    pub id: Option<u64>,
    pub name: Option<String>,
}
