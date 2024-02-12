mod rand;

use gtk::glib::DateTime;

pub fn current_datetime() -> DateTime {
    if let Ok(datetime) = DateTime::now_local() {
        return datetime;
    } else {
        return DateTime::from_unix_utc(0).expect("DateTime is Broken");
    }
}

pub fn number_of_the_day() -> u8 {
    (rand::random_from_seed(date_to_seed(current_datetime())) % 100) as u8
}

pub fn personal_number_of_the_day(name_in: &str) -> u8 {
    let name = name_in.trim().to_lowercase();
    if &name == "" || &name == "name" {
        return number_of_the_day();
    }
    ((rand::random_from_seed(date_to_seed(current_datetime()))
        .wrapping_add(rand::seed_from_name(&name)))
        % 100) as u8
}

fn date_to_seed(date: DateTime) -> u64 {
    format!("{}{}{}", date.day_of_month(), date.month(), date.year())
        .parse()
        .unwrap_or(0)
}
