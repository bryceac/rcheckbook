use bcheck::{ Record, TransactionType, Transaction };
use clap::Parser;
use crate::database::*;

#[derive(Parser)]
#[clap(version = "0.3", author = "Bryce Campbell")]
pub struct Add {

    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    #[clap(long)]
    pub date: Option<String>,

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
    pub fn run(&self) {
        copy_database_if_not_exists(&self.file_path);
        self.add_record(&self.file_path);
    }

    fn add_record(&self, p: &str) {
        let record = Record::from("", Transaction::from(self.date.as_deref(), self.check_number, self.category.as_deref(), &self.vendor, &self.memo, self.amount, self.transaction_type.clone(), self.reconciled).unwrap());

        add_record_to_db(p, &record);
    }
}