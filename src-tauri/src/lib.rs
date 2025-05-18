use chrono::{Datelike, Duration, NaiveDate, Weekday};
use rand::Rng;

#[tauri::command]
fn generate_date() -> String {
    let mut rng = rand::rng();
    let rand_date = random_date_in_range(
        &mut rng,
        NaiveDate::from_ymd_opt(1600, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2099, 12, 31).unwrap(),
    );

    format!("{}", rand_date)
}

#[tauri::command(rename_all = "snake_case")]
fn check_answer(answer: &str, rand_date: &str) -> String {
    let weekday = match answer.trim().parse::<Weekday>() {
        Ok(weekday) => weekday,
        Err(_) => return format!("Error, invalid weekday {}", answer),
    };

    let rand_date = match rand_date.trim().parse::<NaiveDate>() {
        Ok(weekday) => weekday,
        Err(_) => return format!("Error, invalid random date {}", rand_date),
    };

    if weekday == rand_date.weekday() {
        return format!("You are correct!");
    }

    format!("Try again!")
}

#[tauri::command]
fn help() -> String {
    format!("\
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
")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![generate_date, check_answer, help])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn random_date_in_range(
    rng: &mut rand::rngs::ThreadRng,
    start: NaiveDate,
    end: NaiveDate,
) -> NaiveDate {
    let days_in_range = (end - start).num_days();
    let random_days: i64 = rng.random_range(0..days_in_range);
    start + Duration::days(random_days)
}
