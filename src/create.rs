use bcheck::Save;
use clap::Parser;
use std::{path::Path, fs::{ self, File } };
use crate::records::Records;

#[derive(Clap)]
pub struct Create {
    file_path: String
}

impl Create {
    pub fn run(&self) -> Result<(), String> {
        if self.file_path.starts_with("~") {
            let modified_path = shellexpand::tilde(&self.file_path).into_owned();
    
            self.create_new_register(&modified_path)            
        } else {
            self.create_new_register(&self.file_path)
        }
    }

    fn create_empty_register(&self, p: &str) -> Result<(), String> {
        let path = Path::new(p);

        if !path.exists() {
            match File::create(p) {
                Err(error) => Err(error.to_string()),
                _ => Ok(())
            }
        } else {
            Err(String::from("File already exists."))
        } 
    }

    fn create_new_register(&self, p: &str) -> Result<(), String> {
        self.create_empty_register(p)?;

        let records = Records::new();

        if self.file_path.starts_with("~") {
            let modified_path = shellexpand::tilde(&self.file_path).into_owned();
    
            match records.sorted_records().save(&modified_path) {
                Err(error) => Err(error.to_string()),
                _ => Ok(())
            }
        } else {
            match fs::canonicalize(self.file_path.clone()) {
                Ok(real_path) => if let Some(file_path) = real_path.to_str() {
                    match records.sorted_records().save(file_path) {
                        Err(error) => Err(error.to_string()),
                        _ => Ok(())
                    }
                } else {
                    Err(String::from("File path could not be recognized"))
                },
                Err(error) => Err(error.to_string())
            }
        }
    }
}