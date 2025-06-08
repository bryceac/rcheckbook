use clap::Parser;
use crate::{ database::*, shared::* };
use bcheck::{ Record, Save, TransactionType };
use qif::{ DateFormat, QIF, Transaction as QIFTransaction, TransactionBuildingError, Section };
use spsheet::{ Book, Cell, ods, Sheet };


#[derive(Parser)]
pub struct Export {

    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    #[clap(long, short)]
    pub output_file: String
}

impl Export {
    pub fn run(&self) {
        self.export();
    }

    fn export(&self) {
        let destination_path = real_path(&self.output_file);
        let records = load_records_from_db(&self.file_path);

        /* if destination_path.ends_with(".bcheck") {
            if let Err(error) = records.save(&destination_path) {
                println!("{}", error);
            }
        } else {
            if let Err(error) = records.save_tsv(&destination_path) {
                println!("{}", error);
            }
        } */

        match destination_path {
            p if p.ends_with(".bcheck") => if let Err(error) = records.save(&p) {
                println!("{}", error);
            },
            p if p.ends_with(".qif") => if let Err(error) = store_to_qif(records).save(&p, &DateFormat::MonthDayFullYear) {
                println!("{}", error);
            },
            _ => if let Err(error) = records.save_tsv(&destination_path) {
                println!("{}", error);
            }
        }
    }
}

fn record_to_qif(record: &Record) -> Result<QIFTransaction, TransactionBuildingError> {
    let format = DateFormat::MonthDayFullYear;
    
    QIFTransaction::builder()
    .set_date(&record.transaction.date.format(format.chrono_str()).to_string(), &format)
    .set_check_number(record.transaction.check_number.unwrap_or(0))
    .set_vendor(&record.transaction.vendor)
    .set_amount(if let TransactionType::Deposit = record.transaction.transaction_type {
        record.transaction.amount.into_inner()
    } else {
        record.transaction.amount.into_inner()*-1.0
    })
    .set_category(&record.transaction.category.clone().unwrap_or("".to_owned()))
    .set_memo(&record.transaction.memo)
    .set_status(if record.transaction.is_reconciled { 
        "*" 
    } else {
        ""
    })
    .build()
}

fn records_to_qif_transactions(records: Vec<Record>) -> Vec<QIFTransaction> {
    records.into_iter().map(|r| record_to_qif(&r))
    .filter(|t| t.is_ok())
    .map(|r| r.unwrap())
    .collect()
}

fn store_to_qif(records: Vec<Record>) -> QIF {
    let mut qif = QIF::builder();

    let mut bank_builder = Section::builder();
    
    bank_builder.set_type("Bank");

    for transaction in records_to_qif_transactions(records.clone()) {
        bank_builder.add_transaction(transaction);
    }

    if let Some(bank) = bank_builder.build() {
        qif.set_field(bank);
    }

    qif.build()
}

fn add_record_to_sheet(record: &Record, row_index: usize, db: &str, sheet: &mut Sheet) {
    let id_cell = Cell::str(&record.id);
    sheet.add_cell(id_cell, row_index, 0);

    let date = format!("{}", record.transaction.date.format("%Y-%m-%d"));

    let date_cell = Cell::date(date);
    sheet.add_cell(date_cell, row_index, 1);

    let check_number_cell = if let Some(check_number) = record.transaction.check_number {
        Cell::str(check_number.to_string())
    } else {
        Cell::str("")
    };

    sheet.add_cell(check_number_cell, row_index, 2);

    let reconciled_cell = if record.transaction.is_reconciled {
        Cell::str("Y")
    } else {
        Cell::str("N")
    };

    sheet.add_cell(reconciled_cell, row_index, 3)

    let category_cell = Cell::str(record.transaction.category.clone().unwrap_or("".to_string()));

    sheet.add_cell(category_cell, row_index, 4);

    let vendor_cell = Cell::str(record.transaction.vendor.clone());

    sheet.add_cell(vendor_cell, row_index, 5);

    let memo_cell = Cell::str(record.transaction.memo.clone());

    sheet.add_cell(memo_cell, row_index, 6);

    let credit_cell = if let TransactionType::Deposit = record.transaction.transaction_type {
        Cell::float(record.transaction.amount.into_inner())
    } else {
        Cell::str("")
    };

    sheet.add_cell(credit_cell, row_index, 7);

    let withdrawal_cell = if let TransactionType::Withdrawal = record.transaction.transaction_type {
        Cell::float(record.transaction.amount.into_inner())
    } else {
        Cell::str("")
    };

    sheet.add_cell(withdrawal_cell, row_index, 8);

    let balance_cell = Cell::float(retrieve_balance_for_record(db, record.clone()));

    sheet.add_cell(balance_cell, row_index, 9)
}