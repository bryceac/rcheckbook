use bcheck::{ Record, Transaction };
use clap::Parser;
use crate::database::*;

#[derive(Parser)]
#[clap(version = "0.4", author = "Bryce Campbell", long_about = "add entry to ledger. \r\n\r\nAdding a new entry can be done like this: \r\n\r\nrcheckbook add --check-number 1260 --vendor \"Sam Hill Credit Union\" -m \"Open Account\" -a 500.0\r\n\r\nIf you want to include a date, you can do so with -d, which expects the date to be in YYYY-MM-DD format.")]
pub struct Add {

    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    #[clap(long, short)]
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

    #[clap(long, short)]
    pub reconciled: bool
}

impl Add {
    pub fn run(&self) {
        copy_database_if_not_exists(&self.file_path);
        self.add_record(&self.file_path);
    }

    fn add_record(&self, p: &str) {
        let transaction = Transaction::builder()
        .set_date(&self.date.clone().unwrap_or(String::default()))
        .set_check_number(self.check_number.unwrap_or(0))
        .set_category(&self.category.clone().unwrap_or(String::default()))
        .set_vendor(&self.vendor)
        .set_memo(&self.memo)
        .set_amount_and_type(self.amount)
        .set_is_reconciled(self.reconciled)
        .build();

        let record = Record::from("", transaction);

        add_record_to_db(p, &record);
    }
}