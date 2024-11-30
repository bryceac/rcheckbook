use clap::Parser;
use crate::records::Records;
use crate::database::*;
use crate::period::*;

#[derive(Parse)]
pub struct Summary {
    #[clap(default_value = "~/.checkbook/register.db")]
    pub file_path: String,

    #[clap(arg_enum, default_value=Period::All)]
    pub period: Period
}

impl Summary {
    pub fn run(&self) {
        copy_database_if_not_exists(&self.file_path);
        let mut record_store = Records::from(load_records_from_db(&self.file_path));
        let mut categories = load_categories_from_db(&self.file_path);

        match self.period {
            Period::Week => {},
            Period::Month => {},
            Period::Quarter => {},
            Period::HalfYear => {},
            Period::Year => {},
            Period::All => {}
        }
    }

    fn display(records: &Vec<Record>, categories: &Vec<String>, period: &Period) {
        let mut report = String::new();

        match self.period {
            Period::Week => report.push_str("WTD Report\r\n-----\r\n"),
            Period::Month => report.push_str("MTD Report \r\n-----\r\n"),
            Period::HalfYear => report.push_str("Week to Date\r\n-----\r\n"),
            Period::Year => report.push_str("Week to Date\r\n-----\r\n"),
            Period::All => report.push_str("Week to Date\r\n-----\r\n")
        }
    }
}