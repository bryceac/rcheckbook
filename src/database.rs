use std::{ fs, path::Path };

pub fn copy_database_if_not_exists(p: &str) {
    if p.starts_with("~") {
        let modified_path = shellexpand::tilde(p).into_owned();

        if !Path::new(&modified_path).exists() {
            let target_path = Path::new(&modified_path);
            let original_path = Path::new("register.db");

            fs::create_dir_all(&target_path.parent().unwrap());

            let _ = fs::copy(original_path, target_path);
        } else {}
    } else {
        if let Ok(real_path) = fs::canonicalize(p) {
            if let Some(file_path) = real_path.to_str() {

                if !real_path.exists() {
                    let original_path = Path::new("register.db");

                    fs::create_dir_all(real_path.as_path().parent().unwrap());

                    let _ = fs::copy(original_path, real_path);
                } else {}
            }
        }
    }
}