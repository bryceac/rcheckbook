use chrono::prelude::*; // 0.4.38
use chrono::Duration;
use std::mem;

pub struct DateRange(DateTime<Local>, DateTime<Local>);

impl Iterator for DateRange {
    type Item = DateTime<Local>;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

impl DateRange {
    fn contains(&self, date: DateTime<Local>) -> bool {
        if date.timestamp() >= self.0.timestamp() && date.timestamp() <= self.1.timestamp() {
            true
        } else {
            false
        }
    }
}