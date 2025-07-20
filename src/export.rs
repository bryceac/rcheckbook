use std::path::Path;

use clap::Parser;
use crate::{ database::*, shared::* };
use bcheck::{ Record, Save, TransactionType };
use qif::{ DateFormat, QIF, Transaction as QIFTransaction, TransactionBuildingError, Section };
use spreadsheet_ods::{ write_ods, OdsError, Sheet, WorkBook };
use xlsxwriter::prelude::*;
use icu_locid::locale;


#[derive(Parser)]
#[clap(version = "0.4.4", author = "Bryce Campbell")]
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

        match destination_path {
            p if p.ends_with(".bcheck") => if let Err(error) = records.save(&p) {
                println!("{}", error);
            },
            p if p.ends_with(".qif") => if let Err(error) = store_to_qif(records).save(&p, &DateFormat::MonthDayFullYear) {
                println!("{}", error);
            },
            ref p if p.ends_with(".ods") => if let Err(error) = write_ods(&mut create_ods_book(records, &self.file_path), Path::new(&destination_path)) {
                match error {
                    OdsError::Io(error) => println!("{}", error),
                    OdsError::Utf8(error) => println!("{}", error),
                    OdsError::Xml(error) => println!("{}", error),
                    OdsError::Zip(error) => println!("{}", error),
                    _ => ()
                }
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

fn create_ods_book(records: Vec<Record>, db: &str) -> WorkBook {
    let mut workbook = WorkBook::new(locale!("en_US"));

    let mut sheet = Sheet::new("Register");

    sheet.set_value(0, 1, "Date");
    sheet.set_value(0, 2, "Check #");
    sheet.set_value(0, 3, "Reconciled");
    sheet.set_value(0, 4, "Category");
    sheet.set_value(0, 5, "Vendor");
    sheet.set_value(0, 6, "Memo");
    sheet.set_value(0, 7, "Credit");
    sheet.set_value(0, 8, "Withdrawal");
    sheet.set_value(0, 9, "Balance");

    for (index, record) in records.iter().enumerate() {
        let row_index = index+1;

        add_record_to_ods_sheet(record, row_index.try_into().expect("Could not cast number"), db, &mut sheet)
    }

    workbook.push_sheet(sheet);

    workbook
}

fn add_record_to_ods_sheet(record: &Record, row_index: u32, db: &str, sheet: &mut Sheet) {
    sheet.set_value(row_index, 0, record.id.clone());
    
    let date_string = format!("{}", record.transaction.date.format("%Y-%m-%d"));
    sheet.set_value(row_index, 1, date_string);

    sheet.set_value(row_index, 2, if let Some(check_number) = record.transaction.check_number {
        format!("{}", check_number)
    } else {
        String::default()
    });

    sheet.set_value(row_index, 3, if record.transaction.is_reconciled {
        "Y"
    } else {
        "N"
    });

    sheet.set_value(row_index, 4, if let Some(category) = &record.transaction.category {
        category.to_owned()
    } else {
        String::default()
    });

    sheet.set_value(row_index, 5, record.transaction.vendor.clone());
    sheet.set_value(row_index, 6, record.transaction.memo.clone());

    let amount_string = format!("{:.2}", record.transaction.amount);

    if let TransactionType::Deposit = record.transaction.transaction_type {
        sheet.set_value(row_index, 7, amount_string)
    } else {
        sheet.set_value(row_index, 8, amount_string)
    };

    sheet.set_value(row_index, 9, retrieve_balance_for_record(db, record.clone()));
}

fn create_xlsx_book(p: &str, records: Vec<Record>, db: &str) -> Result<(), XlsxError> {
    let workbook = Workbook::new(p)?;

    let mut sheet = workbook.add_worksheet(None)?;

    sheet.write_blank(0, 0, None)?;
    sheet.write_string(0, 1, "Date", None)?;
    sheet.write_string(0, 2, "Check #", None)?;
    sheet.write_string(0, 3, "Reconciled", None)?;
    sheet.write_string(0, 4, "Category", None)?;
    sheet.write_string(0, 5, "Vendor", None)?;
    sheet.write_string(0, 6, "Memo", None)?;
    sheet.write_string(0, 7, "Credit", None)?;
    sheet.write_string(0, 8, "Withdrawal", None)?;
    sheet.write_string(0, 9, "Balance", None)?;

    workbook.close()?;

    Ok(())
}

fn add_record_to_xlsx_sheet(record: &Record, row_index: u32, db: &str, sheet: &mut Worksheet) -> Result<(), XlsxError> {
    sheet.write_string(row_index, 0, &record.id, None)?;

    let date_string = format!("{}", record.transaction.date.format("%Y-%m-%d"));
    sheet.write_string(row_index, 1, &date_string, None)?;

    let check_number = if let Some(check_number) = record.transaction.check_number {
        format!("{}", check_number)
    } else {
        String::default()
    }

    sheet.write_string(row_index, 2, &check_number, None)?;

    sheet.write_string(row_index, 3, if record.transaction.is_reconciled {
        "Y"
    } else {
        "N"
    }, None)?;

    sheet.write_string(row_index, 4, if let Some(category) = &record.transaction.category {
        category
    } else {
        &String::default()
    }, None)?;

    sheet.write_string(row_index, 5, &record.transaction.vendor, None)?;
    sheet.write_string(row_index, 6, &record.transaction.memo, None)?;

    let amount_string = format!("{:.2}", record.transaction.amount);

    if let TransactionType::Deposit = record.transaction.transaction_type {
        sheet.write_string(row_index, 7, &amount_string, None)?
    } else {
        sheet.write_string(row_index, 8, &amount_string, None)?
    };

    sheet.write_number(row_index, 9, retrieve_balance_for_record(db, record.clone()), None)?;

    Ok(())
}

/* fn create_book(records: Vec<Record>, db: &str) -> Book {
    let mut book = Book::new();
    let mut sheet = Sheet::new("Register");

    sheet.add_cell(Cell::str("", ""), 0, 0);
    sheet.add_cell(Cell::str("Date", ""), 0, 1);
    sheet.add_cell(Cell::str("Check #", ""), 0, 2);
    sheet.add_cell(Cell::str("Reconciled", ""), 0, 3);
    sheet.add_cell(Cell::str("Category", ""), 0, 4);
    sheet.add_cell(Cell::str("Vendor", ""), 0, 5);
    sheet.add_cell(Cell::str("Memo", ""), 0, 6);
    sheet.add_cell(Cell::str("Credit", ""), 0, 7);
    sheet.add_cell(Cell::str("Withdrawal", ""), 0, 8);
    sheet.add_cell(Cell::str("Balance", ""), 0, 9);

    for (index, record) in records.iter().enumerate() {
        let row_index = index+1;

        add_record_to_sheet(record, row_index, db, &mut sheet);
    }

    book.add_sheet(sheet);

    book
}

fn add_record_to_sheet(record: &Record, row_index: usize, db: &str, sheet: &mut Sheet) {
    let id_cell = Cell::str(record.id.clone(), "".to_owned());
    sheet.add_cell(id_cell, row_index, 0);

    let date = format!("{}", record.transaction.date.format("%Y-%m-%d"));

    let date_cell = Cell::str(date, "YYYY/MM/DD".to_owned());
    sheet.add_cell(date_cell, row_index, 1);

    let check_number_cell = if let Some(check_number) = record.transaction.check_number {
        Cell::str(check_number.to_string(), "".to_owned())
    } else {
        Cell::str("", "")
    };

    sheet.add_cell(check_number_cell, row_index, 2);

    let reconciled_cell = if record.transaction.is_reconciled {
        Cell::str("Y", "")
    } else {
        Cell::str("N", "")
    };

    sheet.add_cell(reconciled_cell, row_index, 3);

    let category_cell = Cell::str(record.transaction.category.clone().unwrap_or("".to_string()), "".to_owned());

    sheet.add_cell(category_cell, row_index, 4);

    let vendor_cell = Cell::str(record.transaction.vendor.clone().replace("&", "&amp;"), "".to_owned());

    sheet.add_cell(vendor_cell, row_index, 5);

    let memo_cell = Cell::str(record.transaction.memo.clone().replace("&", "&amp;"), "".to_owned());

    sheet.add_cell(memo_cell, row_index, 6);

    let credit_cell = if let TransactionType::Deposit = record.transaction.transaction_type {
        Cell::float(record.transaction.amount.into_inner(), "")
    } else {
        Cell::str("", "")
    };

    sheet.add_cell(credit_cell, row_index, 7);

    let withdrawal_cell = if let TransactionType::Withdrawal = record.transaction.transaction_type {
        Cell::float(record.transaction.amount.into_inner(), "")
    } else {
        Cell::str("", "")
    };

    sheet.add_cell(withdrawal_cell, row_index, 8);

    let balance_cell = Cell::float(retrieve_balance_for_record(db, record.clone()), "");

    sheet.add_cell(balance_cell, row_index, 9)
} */