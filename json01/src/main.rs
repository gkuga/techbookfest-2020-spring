use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Spec {
    pub root: Root,
}

#[derive(Serialize, Deserialize, Debug)]
struct Root {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub readonly: bool,
}

fn main() {
    let point = Spec { root: Root {path: String::from("rootfs"), readonly: true} };

    let serialized = serde_json::to_string(&point).unwrap();
    println!("serialized = {}", serialized);

    let deserialized: Spec = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
