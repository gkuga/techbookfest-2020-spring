extern crate nix;
#[macro_use]
extern crate lazy_static;

use nix::unistd;
use nix::mount::MsFlags;

use std::fs::File;
use std::io::BufReader;

mod mounts;
mod nix_ext;
mod spec;
mod utils;

fn main() {
    let file = File::open("config.json").unwrap();
    let reader = BufReader::new(file);

    let spec: spec::Spec = serde_json::from_reader(reader).unwrap();
    let rootfs = spec.root.path;

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

    nix::mount::mount(
        None::<&str>,
        "/",
        None::<&str>,
        MsFlags::MS_SLAVE | MsFlags::MS_REC,
        None::<&str>,
    )
    .unwrap();

    for m in &spec.mounts {
        let (flags, data) = mounts::parse_mount(m);
        let dest = format! {"{}{}", &rootfs, &m.destination};
        std::fs::create_dir_all(&dest).unwrap();
        nix::mount::mount(Some(&*m.source), &*dest, Some(&*m.typ), flags, Some(&*data)).unwrap();
    }

    unistd::chroot(&rootfs[..]).unwrap();
    unistd::chdir(&rootfs[..]).unwrap();

    utils::do_exec(&spec.process.args[0], &spec.process.args, &spec.process.env);
}
