use bcheck::{ Save };
use clap::Parser;
use crate::records::Records;
use std::fs;

#[derive(Parser)]
pub struct Remove {
    pub file_path: String,

    #[clap(long, short)]
    pub id: String
}

impl Remove {
    pub fn run(&self) -> Result<(), String> {
        if self.file_path.starts_with("~") {
            let modified_path = shellexpand::tilde(&self.file_path).into_owned();
    
            self.remove_record(&modified_path)
        } else {
            match fs::canonicalize(self.file_path.clone()) {
                Ok(real_path) => if let Some(file_path) = real_path.to_str() {
                    self.remove_record(file_path)
                } else {
                    Err(String::from("File path could not be recognized"))
                },
                Err(error) => Err(error.to_string())
            }
        }
    }

    fn remove_record(&self, p: &str) -> Result<(), String> {
        let mut stored_records = Records::from_file(&self.file_path)?;

        stored_records.remove(self.id.clone());

        match stored_records.sorted_records().save(p) {
            Ok(()) => Ok(()),
            Err(error) => Err(error.to_string())
        }
    }
}