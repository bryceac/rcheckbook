use bcheck::Record;

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

    pub fn add(&mut self, record: Record) {
        self.records.push(record)
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

    /* fn record_before(&self, record: &Record) -> Option<Record> {
        if let Some(index) = self.sorted_records().iter().position(|r| r == record) {
            if index > 0 { Some(self.sorted_records()[index-1].clone()) } else { None }
        } else {
            None
        }
    } */

    pub fn record_matching_id(&self, id: String) -> Option<Record> {
        if let Some(record) = self.records.iter().find(|r| r.id == id) {
            Some(record.clone())
        } else {
            None
        }
    }

    pub fn display(&mut self) {
        for record in self.sorted_records() {
            println!("{}", record)
        }
    }
}