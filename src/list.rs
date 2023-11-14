use clap::Parser;
use crate::records::Records;
use bcheck::{ Record, Transaction, TransactionType };
use crate::database::copy_database_if_not_exists;


#[derive(Parser)]
pub struct List {

    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,
}

impl List {
    pub fn run(&self) {
        /* if self.file_path.starts_with("~") {
            let modified_path = shellexpand::tilde(&self.file_path).into_owned();
    
            self.load_from(&modified_path)
        } else {
            if let Ok(real_path) = fs::canonicalize(self.file_path.clone()) {
                if let Some(file_path) = real_path.to_str() {
                    self.load_from(file_path)
                }
            }
        } */
        copy_database_if_not_exists(&self.file_path);
        load_records_from_db(&self.file_path);
    }
}

