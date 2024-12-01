use bcheck::Record;
use crate::database::*;

pub struct Records {
    records: Vec<Record>
}

impl Records {
    pub fn from(r: Vec<Record>) -> Records {
        Records { 
            records: r 
        }
    }
    
    pub fn sorted_records(&self) -> Vec<Record> {
        let mut stored_records = self.records.clone();
        stored_records.sort_by_key(|r| r.transaction.date);
        
        stored_records
    }

    pub fn balance_for_record(&self, db: &str, r: &Record) -> f64 {
        retrieve_balance_for_record(db, r.clone())
    }
}