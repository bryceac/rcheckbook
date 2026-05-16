use chrono::prelude::*; // 0.4.38
use chrono::Duration;
use std::mem;

pub struct DateRange {
    from_date: NaiveDate,
    to_date: NaiveDate
}

impl Iterator for DateRange {
    type Item = NaiveDate;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.from_date <= self.to_date {
            let next = self.from_date + Duration::days(1);
            Some(mem::replace(&mut self.from_date, next))
        } else {
            None
        }
    }
}

impl DateRange {
    pub fn from(f: NaiveDate, t: NaiveDate) -> Self {
        Self {
            from_date: f,
            to_date: t
        }
    }

    pub fn contains(&self, date: NaiveDate) -> bool {
        let from_datetime = Local.from_local_datetime(&self.from_date.and_hms_opt(0, 0, 0).unwrap()).unwrap();
        let to_datetime = Local.from_local_datetime(&self.to_date.and_hms_opt(0, 0, 0).unwrap()).unwrap();
        let query_datetime = Local.from_local_datetime(&date.and_hms_opt(0, 0, 0).unwrap()).unwrap();
        if query_datetime.timestamp() >= from_datetime.timestamp() && query_datetime.timestamp() <= to_datetime.timestamp() {
            true
        } else {
            false
        }
    }
}