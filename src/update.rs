use bcheck::{ LocalDateTimeStringExt, TransactionType, OrderedFloat };
use crate::{database::*, shared::* };
use clap::Parser;

#[derive(Parser)]
pub struct Update {
    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,
    
    #[clap(long, short)]
    pub id: String,

    #[clap(long, short)]
    pub date: Option<String>,

    #[clap(long)]
    pub check_number: Option<u32>,

    #[clap(long)]
    pub category: Option<String>,

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
    pub fn run(&self) {
        
    }

    fn update_record(&self, p: &str) {

        if let Some(mut stored_record) = retrieve_record_with_id_from_db(p, &self.id) {
            if let Some(date) = &self.date {
                match date.local_datetime() {
                    Ok(new_date) => stored_record.transaction.date = new_date,
                    Err(error) => {
                        println!("{}", error);
                        return;
                    }
                }
            }

            if let Some(check_number) = self.check_number {
                stored_record.transaction.check_number = Some(check_number);
            }

            if let Some(vendor) = &self.vendor {
                stored_record.transaction.vendor = vendor.to_owned();
            }

            if let Some(memo) = &self.memo {
                stored_record.transaction.memo = memo.to_owned();
            }

            if let Some(amount) = self.amount {
                stored_record.transaction.amount = OrderedFloat(amount);
            }

            if let Some(transaction_type) = self.transaction_type.clone() {
                stored_record.transaction.transaction_type = transaction_type;
            }

            if self.reconciled {
                stored_record.transaction.is_reconciled = true
            } else if self.not_reconciled {
                stored_record.transaction.is_reconciled = false
            } else {}
        }

        /* if let Some(mut record) = stored_records.record_matching_id(self.id.clone()) {
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
        } */
    }
}