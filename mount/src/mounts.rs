use std::collections::HashMap;

use nix::mount::MsFlags;

use crate::spec::Mount;

#[cfg_attr(rustfmt, rustfmt_skip)]
lazy_static! {
    static ref OPTIONS: HashMap<&'static str, (bool, MsFlags)> = {
        let mut m = HashMap::new();
        m.insert("defaults",      (false, MsFlags::empty()));
        m.insert("ro",            (false, MsFlags::MS_RDONLY));
        m.insert("rw",            (true,  MsFlags::MS_RDONLY));
        m.insert("suid",          (true,  MsFlags::MS_NOSUID));
        m.insert("nosuid",        (false, MsFlags::MS_NOSUID));
        m.insert("dev",           (true,  MsFlags::MS_NODEV));
        m.insert("nodev",         (false, MsFlags::MS_NODEV));
        m.insert("exec",          (true,  MsFlags::MS_NOEXEC));
        m.insert("noexec",        (false, MsFlags::MS_NOEXEC));
        m.insert("sync",          (false, MsFlags::MS_SYNCHRONOUS));
        m.insert("async",         (true,  MsFlags::MS_SYNCHRONOUS));
        m.insert("dirsync",       (false, MsFlags::MS_DIRSYNC));
        m.insert("remount",       (false, MsFlags::MS_REMOUNT));
        m.insert("mand",          (false, MsFlags::MS_MANDLOCK));
        m.insert("nomand",        (true,  MsFlags::MS_MANDLOCK));
        m.insert("atime",         (true,  MsFlags::MS_NOATIME));
        m.insert("noatime",       (false, MsFlags::MS_NOATIME));
        m.insert("diratime",      (true,  MsFlags::MS_NODIRATIME));
        m.insert("nodiratime",    (false, MsFlags::MS_NODIRATIME));
        m.insert("bind",          (false, MsFlags::MS_BIND));
        m.insert("rbind",         (false, MsFlags::MS_BIND | MsFlags::MS_REC));
        m.insert("unbindable",    (false, MsFlags::MS_UNBINDABLE));
        m.insert("runbindable",   (false, MsFlags::MS_UNBINDABLE | MsFlags::MS_REC));
        m.insert("private",       (false, MsFlags::MS_PRIVATE));
        m.insert("rprivate",      (false, MsFlags::MS_PRIVATE | MsFlags::MS_REC));
        m.insert("shared",        (false, MsFlags::MS_SHARED));
        m.insert("rshared",       (false, MsFlags::MS_SHARED | MsFlags::MS_REC));
        m.insert("slave",         (false, MsFlags::MS_SLAVE));
        m.insert("rslave",        (false, MsFlags::MS_SLAVE | MsFlags::MS_REC));
        m.insert("relatime",      (false, MsFlags::MS_RELATIME));
        m.insert("norelatime",    (true,  MsFlags::MS_RELATIME));
        m.insert("strictatime",   (false, MsFlags::MS_STRICTATIME));
        m.insert("nostrictatime", (true,  MsFlags::MS_STRICTATIME));
        m
    };
}

pub fn parse_mount(m: &Mount) -> (MsFlags, String) {
    let mut flags = MsFlags::empty();
    let mut data = Vec::new();
    for s in &m.options {
        match OPTIONS.get(s.as_str()) {
            Some(x) => {
                let (clear, f) = *x;
                if clear {
                    flags &= !f;
                } else {
                    flags |= f;
                }
            }
            None => {
                data.push(s.as_str());
            }
        };
    }
    (flags, data.join(","))
}
