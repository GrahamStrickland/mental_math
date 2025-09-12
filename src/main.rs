use rand::Rng;
use std::convert::{TryFrom, TryInto};
use std::io;

use chrono::{Datelike, NaiveDate, Weekday};
use mental_math::{calendar_dates_help, cross_multiplication_help, random_date_in_range};

static MAX_BIG_NUMBER: i32 = 99999;
static MAX_SMALL_NUMBER: i32 = 99;

enum Choices {
    CalendarDates = 1,
    Multiplication,
}

impl TryFrom<u32> for Choices {
    type Error = ();

    fn try_from(i: u32) -> Result<Self, Self::Error> {
        match i {
            x if x == Choices::CalendarDates as u32 => Ok(Choices::CalendarDates),
            x if x == Choices::Multiplication as u32 => Ok(Choices::Multiplication),
            _ => Err(()),
        }
    }
}

fn main() {
    println!(
        "\
===========
MENTAL MATH
===========
Choose an exercise:
1 = Calculate Calendar Dates 
2 = Cross Multiplication"
    );

    let mut choice = String::new();

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice: u32 = choice.trim().parse().expect("Please type a number!");

    let mut rng = rand::rng();

    match choice.try_into() {
        Ok(Choices::CalendarDates) => calculate_calendar_dates(&mut rng),
        Ok(Choices::Multiplication) => cross_multiplication(&mut rng),
        Err(_) => eprintln!("Unknown option, please select either 1 or 2"),
    }
}

fn calculate_calendar_dates(rng: &mut rand::rngs::ThreadRng) {
    println!("Enter the day of the week:");

    let rand_date = random_date_in_range(
        rng,
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
            Err(_) => {
                if answer.trim() == "help" || answer.trim() == "--help" || answer.trim() == "-h" {
                    calendar_dates_help();
                }
                continue;
            }
        };

        println!("Your answer: {weekday}");

        if weekday == rand_date.weekday() {
            println!("You are correct!");
            break;
        }
        println!("Try again!");
    }
}

fn cross_multiplication(rng: &mut rand::rngs::ThreadRng) {
    println!("Enter the answer:");

    let big_number = rng.random_range(0..MAX_BIG_NUMBER);
    let small_number = rng.random_range(0..MAX_SMALL_NUMBER);

    loop {
        println!("{} x {}", big_number, small_number);

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let answer: i32 = match answer.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                if answer.trim() == "help" || answer.trim() == "--help" || answer.trim() == "-h" {
                    cross_multiplication_help();
                }
                continue;
            }
        };

        println!("Your answer: {answer}");

        if answer == big_number * small_number {
            println!("You are correct!");
            break;
        }
        println!("Try again!");
    }
}
