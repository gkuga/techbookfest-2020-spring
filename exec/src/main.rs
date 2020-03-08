extern crate nix;

use nix::unistd;

use std::fs::File;
use std::io::BufReader;

mod nix_ext;
mod spec;
mod utils;

fn main() {
    let file = File::open("config.json").unwrap();
    let reader = BufReader::new(file);

    let spec: spec::Spec = serde_json::from_reader(reader).unwrap();

    let rootfs = spec.root.path;
    unistd::chroot(&rootfs[..]).unwrap();
    unistd::chdir(&rootfs[..]).unwrap();

    utils::do_exec(&spec.process.args[0], &spec.process.args, &spec.process.env);
}
