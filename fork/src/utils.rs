use std::ffi::{CStr, CString};

use crate::nix_ext::{clearenv, putenv};

pub fn do_exec(path: &str, args: &Vec<String>, env: &Vec<String>) {
    let args_cstring: Vec<_> = args
        .iter()
        .map(|s| CString::new(s.to_string()).unwrap())
        .collect();
    let args_cstr: Vec<&CStr> = args_cstring.iter().map(|c| c.as_c_str()).collect();
    let env_cstring: Vec<_> = env
        .iter()
        .map(|s| CString::new(s.to_string()).unwrap())
        .collect();
    let path_cstring = CString::new(path.to_string()).unwrap();

    clearenv().unwrap();
    for e in &env_cstring {
        putenv(e).unwrap();
    }

    nix::unistd::execvp(&path_cstring, &args_cstr).unwrap();
}
