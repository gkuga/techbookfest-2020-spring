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

    let linux = &spec.linux.as_ref().unwrap();
    let mut cf = nix::sched::CloneFlags::empty();
    for ns in &linux.namespaces {
        let space = nix::sched::CloneFlags::from_bits_truncate(ns.typ as i32);
        cf |= space;
    }
    nix::sched::unshare(cf).unwrap();

    match nix::unistd::fork().unwrap() {
        nix::unistd::ForkResult::Child => {}
        nix::unistd::ForkResult::Parent { child } => {
            match nix::sys::wait::waitpid(child, None) {
                Ok(status) => println!("Child exited ({:?}).", status),
                Err(_) => println!("waitpid() failed"),
            }
            std::process::exit(0);
        }
    };

    utils::do_exec(&spec.process.args[0], &spec.process.args, &spec.process.env);
}
