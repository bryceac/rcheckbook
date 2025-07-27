use clap::Parser;
use bcheck::{Record, TransactionType};
use crate::records::Records;
use crate::database::*;


#[derive(Parser)]
#[clap(version = "0.4", author = "Bryce Campbell", long_about = "Display the ledger. \r\n\r\nTo display the ledger, you can do something as simple as this: \r\n\r\nrcheckbook list \r\n\r\nWith the exception of a balance column, this will display the same data as if you did a TSV export and display everything. \r\n\r\nYou can also get filtered results with the c flag like this: \r\n\r\nrcheckbook list -c Utilities \r\n\r\nThis will list everything in the Utilities category. \r\n\r\nYou can filter by vendor and memo in the same capacity by using the -v and -m flags instead of or in addition to -c. \r\n\r\nFinally, -r and --unreconciled will filter out reconciled or unreconciled transaction, to see what you have reconciled or left to be accounted for.")]
pub struct List {

    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub category: Option<String>,

    #[clap(long, short)]
    pub vendor: Option<String>,

    #[clap(long, short)]
    pub memo: Option<String>,

    #[clap(long, short)]
    pub reconciled: bool,

    #[clap(long)]
    pub not_reconciled: bool,

    #[clap(long, short)]
    pub transaction_type: Option<TransactionType>
}

impl List {
    pub fn run(&self) {
        copy_database_if_not_exists(&self.file_path);
        let record_store = Records::from(load_records_from_db(&self.file_path));

        if self.reconciled && self.not_reconciled {
            print!("Please use only one flag!\r\nNo transactions can be both reconciled and unreconciled.\r\n")
        } else {
            display(&record_store, &self.category, &self.vendor, &self.memo, &self.reconciled, &self.not_reconciled, &self.file_path)
        }  
    }
}

fn retrieve_records(r: &Vec<Record>, category: &Option<String>, vendor: &Option<String>, memo: &Option<String>, reconciled: &bool, unreconciled: &bool, transaction_type: Option<TransactionType>) -> Vec<Record> {
    let mut filtered_records: Vec<Record> = r.clone();
    if let Some(category) = category {
        filtered_records = filtered_records.into_iter().filter(|record| record.transaction.category.clone().unwrap_or("Uncategorized".to_string()).to_lowercase() == category.to_string().to_lowercase()).collect();
    }

    if let Some(vendor) = vendor {
        filtered_records = filtered_records.into_iter().filter(|record| record.transaction.vendor.to_lowercase() == vendor.to_lowercase() || record.transaction.vendor.to_lowercase().contains(&vendor.to_string().to_lowercase())).collect();
    }

    if let Some(memo) = memo {
        filtered_records = filtered_records.into_iter().filter(|record| record.transaction.memo.to_lowercase() == memo.to_lowercase() || record.transaction.memo.to_lowercase().contains(&memo.to_string().to_lowercase())).collect();
    }

    if reconciled.to_owned() {
        filtered_records = filtered_records.into_iter().filter(|record| record.transaction.is_reconciled).collect();
    } else if unreconciled.to_owned() {
        filtered_records = filtered_records.into_iter().filter(|record| !record.transaction.is_reconciled).collect();
    }

    match transaction_type {
        TransactionType::Deposit => filtered_records = filtered_records,into_iter().filter(|record| record.transaction.transaction_type == TransactionType::Deposit),
        TransactionType::Withdrawal => filtered_records = filtered_records,into_iter().filter(|record| record.transaction.transaction_type == TransactionType::Withdrawal),
        _ => ()
    }

    return filtered_records;
}

fn display(store: &Records, category: &Option<String>, vendor: &Option<String>, memo: &Option<String>, reconciled: &bool, unreconciled: &bool, transaction_type: Option<TransactionType>, db: &str) {
    for record in retrieve_records(&store.sorted_records(), category, vendor, memo, reconciled, unreconciled, transaction_type) {
        let balance = store.balance_for_record(db, &record);
        println!("{}\t{:.2}", record, balance);
    }
}

