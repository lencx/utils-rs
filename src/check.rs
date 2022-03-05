use which::which;
use std::path::Path;
use std::fs;

pub fn check_cmd(program: &str) -> bool {
    match which(program) {
        Err(_) => false,
        _ => true,
    }
}

pub fn check_path<P: AsRef<Path>>(path: P) -> bool {
    fs::metadata(path).is_ok()
}
