use std::env;
use std::io;

use chrono::{Datelike, NaiveDate, Weekday};
use mental_math::{help, random_date_in_range};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && &args[1] == "help" {
        help();
        return
    }

    println!("Enter the day of the week:");

    let mut rng = rand::rng();
    let rand_date = random_date_in_range(
        &mut rng,
        NaiveDate::from_ymd_opt(1600, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2099, 12, 31).unwrap(),
    );

    loop {
        println!("Date: {}", rand_date);

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let weekday = match answer.trim().parse::<Weekday>() {
            Ok(weekday) => weekday,
            Err(_) => continue,
        };

        println!("Your answer: {weekday}");

        if weekday == rand_date.weekday() {
            println!("You are correct!");
            break;
        }
        println!("Try again!");
    }
}
