use std::fs;

pub fn real_path(p: &str) -> String {
    if p.starts_with("~") {
        shellexpand::tilde(p).into_owned()
    } else {
        fs::canonicalize(p).unwrap().as_path().to_str().unwrap().to_owned()
    }
}