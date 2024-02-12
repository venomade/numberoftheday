// NOTE: Wrote this before implementing GTK Stuff so is useless now, but I like it so it stays
use std::time::{SystemTime, UNIX_EPOCH};

pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: u32,
}

impl Date {
    fn new(day: u32, month: u32, year: u32) -> Date {
        Date { day, month, year }
    }
}

pub fn current_date() -> Date {
    seconds_to_date(seconds_since_epoch())
}

fn seconds_since_epoch() -> u64 {
    let now = SystemTime::now();
    let since_epoch = now
        .duration_since(UNIX_EPOCH)
        .expect("The fabric of space and time has been disrupted, probably try and fix that");
    since_epoch.as_secs()
}

fn seconds_to_date(seconds: u64) -> Date {
    let mut days_since_epoch = seconds / (60 * 60 * 24);
    let mut year = 1970;
    let mut days_in_year = if is_leap(year) { 366 } else { 365 }; // Replace with 365/366 based on
                                                                  // 1970

    while days_since_epoch >= days_in_year {
        days_since_epoch -= days_in_year;
        year += 1;
        days_in_year = if is_leap(year) { 366 } else { 365 };
    }

    let days_in_month = [
        31,                                  // January
        if is_leap(year) { 29 } else { 28 }, // February
        31,                                  // March
        30,                                  // April
        31,                                  // May
        30,                                  // June
        31,                                  // July
        31,                                  // August
        30,                                  // September
        31,                                  // October
        30,                                  // November
        31,                                  // December
    ];
    let mut month = 0;

    while days_since_epoch >= days_in_month[month as usize] {
        days_since_epoch -= days_in_month[month as usize];
        month += 1;
    }

    Date::new((days_since_epoch + 1) as u32, month + 1, year)
}

fn is_leap(year: u32) -> bool {
    year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}
