use chrono::prelude::*; // 0.4.38
use chrono::Duration;
use std::mem;

pub struct DateRange {
    from_date: DateTime<Local>,
    to_date: DateTime<Local>
}

impl Iterator for DateRange {
    type Item = DateTime<Local>;
    
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
    pub fn from(f: DateTime<Local>, t: DateTime<Local>) -> Self {
        Self {
            from_date: f,
            to_date: t
        }
    }

    pub fn contains(&self, date: DateTime<Local>) -> bool {
        if date.timestamp() >= self.from_date.timestamp() && date.timestamp() <= self.to_date.timestamp() {
            true
        } else {
            false
        }
    }
}