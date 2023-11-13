use std::{ fs, path::Path };

pub fn copy_database_if_not_exists(p: &str) {
    if p.starts_with("~") {
        let modified_path = shellexpand::tilde(p).into_owned();

        if !Path::new(&modified_path).exists() {
            let target_path = Path::new(&modified_path);
            let original_path = Path::new("register.db");

            fs::copy(original_path, target_path);
        } else {}
    } else {
        if let Ok(real_path) = fs::canonicalize(p) {
            if let Some(file_path) = real_path.to_str() {
                let target_path = Path::new(&file_path);
                let original_path = Path::new("register.db");

                let _ = fs::copy(original_path, target_path);
            }
        }
    }
}