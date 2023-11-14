use bcheck::{ Record, TransactionType, Transaction, Save };
use clap::Parser;
use crate::{ database::*, records::Records };
use std::fs;

#[derive(Parser)]
pub struct Add {

    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    #[clap(long)]
    pub check_number: Option<u32>,

    #[clap(long)]
    pub category: Option<String>,

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
        copy_database_if_not_exists(&self.file_path);
        self.add_record(&self.file_path)
    }

    fn add_record(&self, p: &str) -> Result<(), String> {
        let mut stored_records = Records::from(load_records_from_db(p));

        let record = Record::from("", Transaction::from(None, self.check_number, None, &self.vendor, &self.memo, self.amount, self.transaction_type.clone(), self.reconciled).unwrap());

        stored_records.add(record);

        match stored_records.sorted_records().save(p) {
            Ok(()) => Ok(()),
            Err(error) => Err(error.to_string())
        }
    }
}