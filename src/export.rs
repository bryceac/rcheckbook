use std::path::Path;

use clap::Parser;
use crate::{ database::*, shared::* };
use bcheck::{ Record, Save, TransactionType };
use qif::{ DateFormat, QIF, Transaction as QIFTransaction, TransactionBuildingError, Section };
use spreadsheet_ods::{ write_ods, OdsError, Sheet, WorkBook };
use xlsxwriter::prelude::*;
use icu_locid::locale;


#[derive(Parser)]
#[clap(version = "0.4.5", author = "Bryce Campbell", long_about = "Export ledger to file. Exports are done based upon given file extension, with support for JSON (bcheck), QIF, Excel (xlsx), ODS, and TSV. Default output is TSV. Excel and ODS output will include headers and a column for balances, unlike TSV. However, TSV files can only be imported when they have the TSV extension.")]
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
            ref p if p.ends_with(".xlsx") => if let Err(error) = create_xlsx_book(&destination_path, records, &self.file_path) {
                println!("{}", error)
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

    for (index, record) in records.iter().enumerate() {
        let row_index = index+1;

        add_record_to_xlsx_sheet(record, row_index.try_into().expect("Could not cast number"), db, &mut sheet)?;
    }

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
    };

    sheet.write_string(row_index, 2, &check_number, None)?;

    sheet.write_string(row_index, 3, if record.transaction.is_reconciled {
        "Y"
    } else {
        "N"
    }, None)?;

    let category = if let Some(category) = &record.transaction.category {
        category.to_owned()
    } else {
        String::default()
    };

    sheet.write_string(row_index, 4, &category, None)?;

    sheet.write_string(row_index, 5, &record.transaction.vendor, None)?;
    sheet.write_string(row_index, 6, &record.transaction.memo, None)?;

    if let TransactionType::Deposit = record.transaction.transaction_type {
        sheet.write_number(row_index, 7, record.transaction.amount.into_inner(), None)?;
        sheet.write_blank(row_index, 8, None)?;
    } else {
        sheet.write_blank(row_index, 7, None)?;
        sheet.write_number(row_index, 8, record.transaction.amount.into_inner(), None)?;
    }

    sheet.write_number(row_index, 9, retrieve_balance_for_record(db, record.clone()), None)?;

    Ok(())
}