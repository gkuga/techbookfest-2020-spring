use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
    pub root: Root,
    pub process: Process,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Root {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub readonly: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Process {
    pub args: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<String>,
}
