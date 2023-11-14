use std::num::FpCategory;

use clap::Parser;
use crate::records::Records;
use bcheck::{ Record, Transaction, TransactionType };
use crate::database::copy_database_if_not_exists;
use rusqlite::Connection;

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
        self.load_from_db(&self.file_path);
    }

    fn load_from_db(&self, p: &str) {
        /* match Records::from_file(p) {
            Ok(mut records) => {
                records.display()
            },
            Err(error) => {
                println!("{}", error)
            }
        } */

        match Connection::open(p) {
            Ok(db) => {
                if let Ok(statement) = db.prepare("SELECT * from ledger") {
                    let record_query = statement.query_map([], |row| {
                        let id: String = row.get_unwrap(0);
                        let date_string: String = row.get_unwrap(1);
                        let category = if let Ok(c) = row.get(6) {
                            Some(c)
                        } else {
                            None
                        };
                        
                        Ok(Record::from(&id, 
                        Transaction::from(Some(&date_string),
                        row.get_unwrap(2), 
                        row.get_unwrap(6), 
                        row.get_unwrap(4), 
                        row.get_unwrap(5), 
                        row.get_unwrap(7), 
                        if let Ok(amount) = row.get(7) {
                            if amount > 0 { 
                                TransactionType::Deposit 
                            } else {
                                TransactionType::Withdrawal
                            }
                        } else {
                            TransactionType::Withdrawal
                        }, 
                        row.get_unwrap(3)).unwrap()))
                    }).unwrap_or("Could not get results");
                }
                let _ = Connection::close(db);
            },
            _ => println!("Could not connect")
        }
    }
}

