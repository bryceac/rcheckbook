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

    pub fn add(&mut self, record: &Record) {
        self.records.push(record.clone())
    }

    pub fn remove(&mut self, record_id: String) {
        if let Some(index) = self.records.iter().position(|r| r.id == record_id) {
            self.records.remove(index);
        }
    }

    pub fn update(&mut self, record_id: String, record: Record) {
        if let Some(index) = self.records.iter().position(|r| r.id == record_id) {
            self.records[index] = record
        }
    }

    pub fn record_matching_id(&self, id: String) -> Option<Record> {
        if let Some(record) = self.records.iter().find(|r| r.id == id) {
            Some(record.clone())
        } else {
            None
        }
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