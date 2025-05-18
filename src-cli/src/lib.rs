use chrono::{Duration, NaiveDate};
use rand::Rng;

pub fn random_date_in_range(
    rng: &mut rand::rngs::ThreadRng,
    start: NaiveDate,
    end: NaiveDate,
) -> NaiveDate {
    let days_in_range = (end - start).num_days();
    let random_days: i64 = rng.random_range(0..days_in_range);
    start + Duration::days(random_days)
}

pub fn help() {
    println!("\
===========================
Calculating Calendar Dates:
===========================
Century:
    1600s / 2000s / 2400s /…  +2
    1700s / 2100s / 2500s /…  +0
    1800s / 2200s / 2600s /…  +5
    1900s / 2300s / 2700s /…  +3

Year:
    Divide the year by 4 and ignore any remainder. Then add this to the original year. Find the remainder when dividing by 7.

Month:
    January:   +4
    February:  +0
    March:     +0
    April:     +3
    May:       +5
    June:      +1
    July:      +3
    August:    +6
    September: +2
    October:   +4
    November:  +0
    December:  +2

Day:
    Just use the date itself. But to simplify calculation later, it is better to find the remainder when dividing by 7.
");
}
