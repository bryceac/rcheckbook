use bcheck::Record;
use crate::database::*;

pub struct Records {
    records: Vec<Record>
}

impl Records {
    pub fn new() -> Records {
        Records {
            records: Vec::new()
        }
    }

    pub fn from(r: Vec<Record>) -> Records {
        Records { 
            records: r 
        }
    }
    
    pub fn from_file(path: &str) -> Result<Records, String> {
        match path {
            p if p.ends_with(".json") => match Record::from_file(path) {
                Ok(records) => Ok(Records { records }),
                Err(error) => Err(error)
            },
            p if p.ends_with(".tsv") => match Record::from_tsv_file(path) {
                Ok(records) => Ok(Records { records }),
                Err(error) => Err(error)
            },
            _ => {
                Ok(Records::new())
            }
        }
    }
    
    pub fn sorted_records(&self) -> Vec<Record> {
        let mut stored_records = self.records.clone();
        stored_records.sort_by_key(|r| r.transaction.date);
        
        stored_records
    }

    pub fn display(&mut self, db: &str) {
        for record in self.sorted_records() {
            let balance = self.balance_for_record(db, &record);
            println!("{}\t{:.2}", record, balance);
        }
    }

    fn balance_for_record(&self, db: &str, r: &Record) -> f64 {
        retrieve_balance_for_record(db, r.clone())
    }
}