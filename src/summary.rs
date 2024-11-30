use bcheck::{ Record, TransactionType };
use clap::Parser;
use crate::records::Records;
use crate::database::*;
use crate::period::*;

#[derive(Parser)]
pub struct Summary {
    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    #[clap(value_enum, default_value_t=Period::All)]
    pub period: Period
}

impl Summary {
    pub fn run(&self) {
        copy_database_if_not_exists(&self.file_path);
        let mut record_store = Records::from(load_records_from_db(&self.file_path));
        let mut categories = load_categories_from_db(&self.file_path);

        match self.period {
            Period::Week => {
                let
            },
            Period::Month => {},
            Period::Quarter => {},
            Period::HalfYear => {},
            Period::Year => {},
            Period::All => {}
        }
    }

    fn display(records: &Vec<Record>, categories: &Vec<String>, period: &Period) {
        let mut report = String::new();
        let filtered_categories: Vec<String> = categories.clone().into_iter().filter(|category| category.to_lowercase() != "Opening Balance".to_string().to_lowercase()).collect();

        match period {
            Period::Week => report.push_str("WTD Report\r\n-----\r\n"),
            Period::Month => report.push_str("MTD Report \r\n-----\r\n"),
            Period::Quarter => report.push_str("QTD Report \r\n-----\r\n"),
            Period::HalfYear => report.push_str("6 Month Report\r\n-----\r\n"),
            Period::Year => report.push_str("YTD Report\r\n-----\r\n"),
            Period::All => report.push_str("Summary\r\n-----\r\n")
        }

        let opening_index = records.iter().position(|record| record.transaction.category.clone().unwrap_or("Uncategorized".to_string()).to_lowercase() == "Opening Balance".to_string().to_lowercase());

        let opening = format!("Opening Balance:\t{:.2}\r\n", if let Some(starting_index) = opening_index {
            records[starting_index].transaction.amount.into_inner()
        } else {
            records[0].transaction.amount.into_inner()
        });

        for category in filtered_categories{
            let records_in_category: Vec<Record> = records.into_iter().filter(|record| record.transaction.category.clone().unwrap_or("Uncategorized".to_string()).to_lowercase() == category.to_lowercase()).map(|r| r.clone()).collect();

            let category_total = records_in_category.into_iter().fold(0.0, |sum, record| {
                if let TransactionType::Withdrawal = record.transaction.transaction_type {
                    sum - record.transaction.amount.into_inner()
                } else {
                    sum + record.transaction.amount.into_inner()
                }
            });

            let entry = format!("{}:\t{}\r\n", category, category_total);

            report.push_str(&entry);
        }
    }
}