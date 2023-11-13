use bcheck::{ Record, TransactionType, Transaction, Save };
use clap::Parser;
use crate::records::Records;
use std::fs;

#[derive(Parser)]
pub struct Add {
    pub file_path: String,

    #[clap(long)]
    pub check_number: Option<u32>,

    #[clap(long)]
    pub vendor: String,

    #[clap(long, short, default_value = "")]
    pub memo: String,

    #[clap(long, short, default_value = "0.0")]
    pub amount: f64,

    #[clap(long, short, default_value = "withdrawal")]
    pub transaction_type: TransactionType,

    #[clap(long, short)]
    pub reconciled: bool
}

impl Add {
    pub fn run(&self) -> Result<(), String> {
        if self.file_path.starts_with("~") {
            let modified_path = shellexpand::tilde(&self.file_path).into_owned();
    
            self.add_record_to(&modified_path)
        } else {
            match fs::canonicalize(self.file_path.clone()) {
                Ok(real_path) => if let Some(file_path) = real_path.to_str() {
                    self.add_record_to(file_path)
                } else {
                    Err(String::from("File path could not be recognized"))
                },
                Err(error) => Err(error.to_string())
            }
        }
    }

    fn add_record_to(&self, p: &str) -> Result<(), String> {
        let mut stored_records = Records::from_file(p)?;

        let record = Record::from("", Transaction::from(None, self.check_number, &self.vendor, &self.memo, self.amount, self.transaction_type.clone(), self.reconciled).unwrap(), None);

        stored_records.add(record);

        match stored_records.sorted_records().save(p) {
            Ok(()) => Ok(()),
            Err(error) => Err(error.to_string())
        }
    }
}