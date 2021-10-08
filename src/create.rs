use bcheck::Save;
use clap::Clap;
use std::fs;
use crate::records::Records;

#[derive(Clap)]
pub struct Create {
    file_path: String
}

impl Create {
    pub fn run(&self) -> Result<(), String> {
        if self.file_path.starts_with("~") {
            let modified_path = shellexpand::tilde(&self.file_path).into_owned();
    
            self.create_empty_register(&modified_path)            
        } else {
            match fs::canonicalize(self.file_path.clone()) {
                Ok(real_path) => if let Some(file_path) = real_path.to_str() {
                    self.create_empty_register(file_path)
                } else {
                    Err(String::from("File path could not be recognized"))
                },
                Err(error) => Err(error.to_string())
            }
        }
    }

    fn create_empty_register(&self, p: &str) -> Result<(), String> {
        let records = Records::new();

        match records.sorted_records().save(p) {
            Ok(()) => Ok(()),
            Err(error) => Err(error.to_string())
        }
    }
}