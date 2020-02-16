use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Spec {
    pub root: Root,
    pub process: Process,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linux: Option<Linux>,
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Linux {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub namespaces: Vec<LinuxNamespace>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LinuxNamespace {
    #[serde(rename = "type")]
    pub typ: LinuxNamespaceType,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
}

#[allow(nonstandard_style)]
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum LinuxNamespaceType {
    mount = 0x00020000,   /* New mount namespace group */
    cgroup = 0x02000000,  /* New cgroup namespace */
    uts = 0x04000000,     /* New utsname namespace */
    ipc = 0x08000000,     /* New ipc namespace */
    user = 0x10000000,    /* New user namespace */
    pid = 0x20000000,     /* New pid namespace */
    network = 0x40000000, /* New network namespace */
}
