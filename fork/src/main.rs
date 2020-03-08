extern crate nix;

use nix::{sched, sys, unistd};

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

    let linux = &spec.linux.as_ref().unwrap();
    let mut cf = sched::CloneFlags::empty();
    for ns in &linux.namespaces {
        let space = sched::CloneFlags::from_bits_truncate(ns.typ as i32);
        cf |= space;
    }
    sched::unshare(cf).unwrap();

    match unistd::fork().unwrap() {
        unistd::ForkResult::Child => {}
        unistd::ForkResult::Parent { child } => {
            match sys::wait::waitpid(child, None) {
                Ok(status) => println!("Child exited ({:?}).", status),
                Err(_) => println!("waitpid() failed"),
            }
            std::process::exit(0);
        }
    };

    utils::do_exec(&spec.process.args[0], &spec.process.args, &spec.process.env);
}
