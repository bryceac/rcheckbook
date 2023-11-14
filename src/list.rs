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
                        let check_number: Option<u32> = if let Ok(num) = row.get(2) {
                            Some(num)
                        } else {
                            None
                        };
                        let category: Option<String> = if let Ok(c) = row.get(6) {
                            Some(c)
                        } else {
                            None
                        };
                        let vendor: String = if let Ok(v) = row.get(4) {
                            v
                        } else {
                            String::default()
                        };

                        let memo: String = if let Ok(m) = row.get(4) {
                            m
                        } else {
                            String::default()
                        };

                        let amount = if let Ok(a) = row.get(7) {
                            a
                        } else {
                            0.0
                        };

                        let transaction_type = if amount > 0.0 {
                            TransactionType::Deposit
                        } else {
                            TransactionType::Withdrawal
                        };

                        let is_reconciled = if let Ok(r: &str) = row.get(3) {
                            String::from(r)
                        } else {
                            String::from("N")
                        };

                        Ok(Record::from(&id, 
                        Transaction::from(Some(&date_string),
                        check_number, 
                        category.as_deref(), 
                        &vendor, 
                        &memo, 
                        amount, 
                        transaction_type, 
                        if is_reconciled == "Y" { true } else { false }).unwrap()))
                    });
                }
                let _ = Connection::close(db);
            },
            _ => println!("Could not connect")
        }
    }
}

