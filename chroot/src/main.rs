extern crate nix;

use nix::unistd;
use serde::{Deserialize, Serialize};

use std::fs::File;
use std::io::BufReader;

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
    let file = File::open("config.json").unwrap();
    let reader = BufReader::new(file);

    let spec: Spec = serde_json::from_reader(reader).unwrap();

    let rootfs = spec.root.path;
    unistd::chroot(&rootfs[..]).unwrap();
    unistd::chdir(&rootfs[..]).unwrap();

    let dir = unistd::getcwd().unwrap();
    println!("現在のディレクトリ: {:?}", dir)
}
