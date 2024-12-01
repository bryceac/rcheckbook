use bcheck::{ Record, TransactionType };
use chrono::Duration;
use chrono::{ Months, prelude::* };
use clap::Parser;
use crate::date_range::DateRange;
use crate::records::Records;
use crate::database::*;
use crate::period::*;

#[derive(Parser)]
pub struct Summary {
    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    #[clap(value_enum, default_value_t=Period::All)]
    pub period: Period,
}

impl Summary {
    pub fn run(&self) {
        copy_database_if_not_exists(&self.file_path);
        let record_store = Records::from(load_records_from_db(&self.file_path));
        let categories = load_categories_from_db(&self.file_path);
        let today = Local::now();

        match self.period {
            Period::Week => {
                let one_week_ago = today - Duration::weeks(1);
                let week_records: Vec<Record> = record_store.sorted_records().into_iter().filter(|record| DateRange::from(one_week_ago, today).contains(record.transaction.date)).collect();

                Self::display(&week_records, &categories, &self.period, &self.file_path);
            },
            Period::Month => {
                let one_month_ago = today - Months::new(1);
                let month_records: Vec<Record> = record_store.sorted_records().into_iter().filter(|record| DateRange::from(one_month_ago, today).contains(record.transaction.date)).collect();

                Self::display(&month_records, &categories, &self.period, &self.file_path);
            },
            Period::Quarter => {
                let three_months_ago = today - Months::new(3);
                let quarter_records: Vec<Record> = record_store.sorted_records().into_iter().filter(|record| DateRange::from(three_months_ago, today).contains(record.transaction.date)).collect();

                Self::display(&quarter_records, &categories, &self.period, &self.file_path);
            },
            Period::HalfYear => {
                let six_months_ago = today - Months::new(6);
                let half_year_records: Vec<Record> = record_store.sorted_records().into_iter().filter(|record| DateRange::from(six_months_ago, today).contains(record.transaction.date)).collect();

                Self::display(&half_year_records, &categories, &self.period, &self.file_path);
            },
            Period::Year => {
                let one_year_ago = today - Months::new(12);
                let year_records: Vec<Record> = record_store.sorted_records().into_iter().filter(|record| DateRange::from(one_year_ago, today).contains(record.transaction.date)).collect();

                Self::display(&year_records, &categories, &self.period, &self.file_path);
            },
            Period::All => Self::display(&record_store.sorted_records(), &categories, &self.period, &self.file_path)
        }
    }

    fn create_string(records: &Vec<Record>, categories: &Vec<String>, period: &Period, path: &str) -> String {
        let mut report = String::new();
        let mut filtered_categories: Vec<String> = categories.clone().into_iter().filter(|category| category.to_lowercase() != "Opening Balance".to_string().to_lowercase()).collect();
        filtered_categories.sort();

        match period {
            Period::Week => report.push_str("WTD Report\r\n\r\n"),
            Period::Month => report.push_str("MTD Report \r\n\r\n"),
            Period::Quarter => report.push_str("QTD Report \r\n\r\n"),
            Period::HalfYear => report.push_str("6 Month Report\r\n\r\n"),
            Period::Year => report.push_str("YTD Report\r\n\r\n"),
            Period::All => report.push_str("Summary\r\n\r\n")
        }

        let opening_index = records.iter().position(|record| record.transaction.category.clone().unwrap_or("Uncategorized".to_string()).to_lowercase() == "Opening Balance".to_string().to_lowercase());

        let opening = format!("Opening Balance\t{:.2}\r\n\r\n", if let Some(starting_index) = opening_index {
            retrieve_balance_for_record(path, records[starting_index].clone())
        } else if !records.is_empty() {
            retrieve_balance_for_record(path, records[0].clone())
        } else {
            0.0
        });

        report.push_str(&opening);

        for category in filtered_categories{
            let records_in_category: Vec<Record> = records.into_iter().filter(|record| record.transaction.category.clone().unwrap_or("Uncategorized".to_string()).to_lowercase() == category.to_lowercase()).map(|r| r.clone()).collect();

            let category_total = records_in_category.into_iter().fold(0.0, |sum, record| {
                if let TransactionType::Withdrawal = record.transaction.transaction_type {
                    sum - record.transaction.amount.into_inner()
                } else {
                    sum + record.transaction.amount.into_inner()
                }
            });

            let entry = format!("{}\t{:.2}\r\n", category, category_total);

            report.push_str(&entry);
        }

        let last_record = records.last();

        let balance = if let Some(last) = last_record {
            retrieve_balance_for_record(path, last.clone())
        } else {
            0.0
        };

        let balance_entry = format!("\r\nBalance\t{:.2}\r\n\r\n", balance);
        report.push_str(&balance_entry);

        let total_income = records.into_iter().filter(|r| r.transaction.transaction_type == TransactionType::Deposit).fold(0.0, |sum, i| sum + i.transaction.amount.into_inner());

        let income_entry = format!("Total Income\t{:.2}\r\n", total_income);
        report.push_str(&income_entry);


        let total_expenses = records.into_iter().filter(|r| r.transaction.transaction_type == TransactionType::Withdrawal).fold(0.0, |sum, i| sum + i.transaction.amount.into_inner());

        let expenditure_entry = format!("Total Expenditures\t{:.2}\r\n\r\n", total_expenses);
        report.push_str(&expenditure_entry);

        let total_reconciled = records.into_iter().filter(|r| r.transaction.is_reconciled).fold(0.0, |sum, i| {
            if let TransactionType::Deposit = i.transaction.transaction_type {
                sum + i.transaction.amount.into_inner()
            } else {
                sum - i.transaction.amount.into_inner()
            }
        });

        let reconciled_entry = format!("Reconciled\t{:.2}\r\n", total_reconciled);
        report.push_str(&reconciled_entry);

        let total_unreconciled = records.into_iter().filter(|r| !r.transaction.is_reconciled).fold(0.0, |sum, i| {
            if let TransactionType::Deposit = i.transaction.transaction_type {
                sum + i.transaction.amount.into_inner()
            } else {
                sum - i.transaction.amount.into_inner()
            }
        });

        let unreconciled_entry = format!("Unreconciled\t{:.2}", total_unreconciled);
        report.push_str(&unreconciled_entry);



        return report;
    }

    fn display(records: &Vec<Record>, categories: &Vec<String>, period: &Period, path: &str) {
        print!("{}", Self::create_string(records, categories, period, path));
    }
}