use clap::Parser;
use std::{ fs, path::Path };
use crate::records::Records;

#[derive(Parser)]
pub struct List {
    pub file_path: String,
}

impl List {
    pub fn run(&self) {
        if self.file_path.starts_with("~") {
            let modified_path = shellexpand::tilde(&self.file_path).into_owned();
    
            self.load_from(&modified_path)
        } else {
            if let Ok(real_path) = fs::canonicalize(self.file_path.clone()) {
                if let Some(file_path) = real_path.to_str() {
                    self.load_from(file_path)
                }
            }
        }
    }

    fn load_from(&self, p: &str) {
        match Records::from_file(p) {
            Ok(mut records) => {
                records.display()
            },
            Err(error) => {
                println!("{}", error)
            }
        }
    }

    fn copy_database_if_not_exists() {
        if self.file_path.starts_with("~") {
            let modified_path = shellexpand::tilde(&self.file_path).into_owned();

            if !Path::new(&modified_path).exists() {
                let target_path = Path::new(&modified_path);
                let original_path = Path::new("register.db");

                fs::copy(original_path, target_path);
            } else {}
        } else {
            if let Ok(real_path) = fs::canonicalize(self.file_path.clone()) {
                if let Some(file_path) = real_path.to_str() {
                    let target_path = Path::new(&file_path);
                    let original_path = Path::new("register.db");

                    fs::copy(original_path, target_path);
                }
            }
        }
    }
}

