use chrono::{Date, Datelike, Local};

const MONTH_NAMES: [&str; 12] = [
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December",
];

#[derive(Copy, Clone)]
pub struct CalDate {
    pub date: Date<Local>,
}
impl CalDate {
    fn new(date: Date<Local>) -> CalDate {
        CalDate { date }
    }

    pub fn next(&mut self) -> Self {
        CalDate::new(self.date.succ())
    }
}
impl std::fmt::Display for CalDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.date)
    }
}

impl From<Date<Local>> for CalDate {
    fn from(d: Date<Local>) -> Self {
        CalDate::new(d)
    }
}

pub struct CalView {
    first: CalDate,
    last: CalDate,
    pub year: i32,
    pub month: &'static str,
}
impl CalView {
    pub fn new(d: Date<Local>) -> CalView {
        let mut first = d.with_day(1).unwrap();
        let mut last = first.with_month(d.month() + 1).unwrap().pred();
        for _ in 0..first.weekday().num_days_from_monday() {
            first = first.pred();
        }
        for _ in last.weekday().num_days_from_monday()..6 {
            last = last.succ();
        }
        CalView {
            first: CalDate::new(first),
            last: CalDate::new(last),
            year: first.year(),
            month: MONTH_NAMES[d.month0() as usize],
        }
    }

    pub fn iter(&self) -> CalIterator {
        CalIterator {
            cal: self,
            curr: self.first,
        }
    }
}
pub struct CalIterator<'a> {
    cal: &'a CalView,
    curr: CalDate,
}
impl<'a> Iterator for CalIterator<'a> {
    type Item = CalDate;

    fn next(&mut self) -> Option<Self::Item> {
        let r = if self.curr.date <= self.cal.last.date {
            Some(self.curr)
        } else {
            None
        };
        self.curr = self.curr.next();
        r
    }
}
