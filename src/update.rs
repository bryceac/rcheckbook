use bcheck::{ LocalDateTimeStringExt, OrderedFloat, TransactionType, Save };
use crate::records::Records;
use clap::Parser;
use std::fs;

#[derive(Clap)]
pub struct Update {
    pub file_path: String,
    
    #[clap(long, short)]
    pub id: String,

    #[clap(long, short)]
    pub date: Option<String>,

    #[clap(long)]
    pub check_number: Option<u32>,

    #[clap(long)]
    pub vendor: Option<String>,

    #[clap(long, short)]
    pub memo: Option<String>,

    #[clap(long, short)]
    pub amount: Option<f64>,

    #[clap(long, short)]
    pub transaction_type: Option<TransactionType>,

    #[clap(long, short)]
    pub reconciled: bool,

    #[clap(long)]
    pub not_reconciled: bool
}

impl Update {
    pub fn run(&self) -> Result<(), String> {
        if self.file_path.starts_with("~") {
            let modified_path = shellexpand::tilde(&self.file_path).into_owned();
    
            self.update_record(&modified_path)
        } else {
            match fs::canonicalize(self.file_path.clone()) {
                Ok(real_path) => if let Some(file_path) = real_path.to_str() {
                    self.update_record(file_path)
                } else {
                    Err(String::from("File path could not be recognized"))
                },
                Err(error) => Err(error.to_string())
            }
        }
    }

    fn update_record(&self, p: &str) -> Result<(), String> {
        let mut stored_records = Records::from_file(p)?;

        if let Some(mut record) = stored_records.record_matching_id(self.id.clone()) {
            if let Some(date_string) = self.date.clone() {
                if let Ok(datetime) = date_string.local_datetime() {
                    record.transaction.date = datetime
                }
            } else {}

            if let Some(check_number) = self.check_number {
                record.transaction.check_number = Some(check_number);
            } else {}
            
            if let Some(vendor) = self.vendor.clone() {
                record.transaction.vendor = vendor;
            } else {}

            if let Some(memo) = self.memo.clone() {
                record.transaction.memo = memo;
            } else {}

            if let Some(amount) = self.amount {
                record.transaction.amount = OrderedFloat(amount);
            } else {}

            if let Some(transaction_type) = self.transaction_type.clone() {
                record.transaction.transaction_type = transaction_type;
            } else {}

            if self.reconciled {
                record.transaction.is_reconciled = true;
            } else if self.not_reconciled {
                record.transaction.is_reconciled = false;
            } else {}

            stored_records.update(self.id.clone(), record);

            match stored_records.sorted_records().save(p) {
                Ok(()) => Ok(()),
                Err(error) => Err(error.to_string())
            }

        } else {
            Err(String::from("Could not find record."))
        }
    }
}