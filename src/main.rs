use std::convert::{TryFrom, TryInto};
use std::io;

use chrono::{Datelike, NaiveDate, TimeDelta, Weekday};
use rand::Rng;
use termcolor::Color;

use mental_math::{
    calendar_dates_help, cross_multiplication_help, print_color, print_error, print_statistics,
    random_date_in_range, timefunc,
};

static MAX_BIG_NUMBER: i32 = 99999;
static MAX_SMALL_NUMBER: i32 = 20;

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
"
    );

    let mut calendar_successes = 0;
    let mut calendar_failures = 0;
    let mut calendar_duration = TimeDelta::zero();
    let mut multiplication_successes = 0;
    let mut multiplication_failures = 0;
    let mut multiplication_duration = TimeDelta::zero();

    loop {
        println!(
            "\
Choose an exercise:
1 = Calculate Calendar Dates 
2 = Cross Multiplication
"
        );
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = choice.trim().parse().expect("Please type a number!");

        let mut rng = rand::rng();

        match choice.try_into() {
            Ok(Choices::CalendarDates) => {
                let (failures, duration) = timefunc(calculate_calendar_dates, &mut rng);
                calendar_failures += failures;
                calendar_successes += 1;
                calendar_duration += duration;
            }
            Ok(Choices::Multiplication) => {
                let (failures, duration) = timefunc(cross_multiplication, &mut rng);
                multiplication_failures += failures;
                multiplication_successes += 1;
                multiplication_duration += duration;
            }
            Err(_) => print_error("Unknown option, please select either 1 or 2."),
        }

        print_statistics(
            calendar_successes,
            calendar_failures,
            calendar_duration,
            multiplication_successes,
            multiplication_failures,
            multiplication_duration,
        );
    }
}

fn calculate_calendar_dates(rng: &mut rand::rngs::ThreadRng) -> u32 {
    let mut failures = 0;

    println!("Enter the day of the week:");

    let rand_date = random_date_in_range(
        rng,
        NaiveDate::from_ymd_opt(1600, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2099, 12, 31).unwrap(),
    );

    loop {
        print_color(format!("Date: {}\n", rand_date).as_str(), Color::Blue);

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

        print_color(format!("Your answer: {weekday}\n").as_str(), Color::Yellow);

        if weekday == rand_date.weekday() {
            print_color("You are correct!\n", Color::Green);
            return failures;
        }
        print_error("Try again!");
        failures += 1;
    }
}

fn cross_multiplication(rng: &mut rand::rngs::ThreadRng) -> u32 {
    let mut failures = 0;

    println!("Enter the answer:");

    let big_number = rng.random_range(0..MAX_BIG_NUMBER);
    let small_number = rng.random_range(0..MAX_SMALL_NUMBER);

    loop {
        print_color(
            format!("{} x {}\n", big_number, small_number).as_str(),
            Color::Blue,
        );

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

        print_color(format!("Your answer: {answer}\n").as_str(), Color::Yellow);

        if answer == big_number * small_number {
            print_color("You are correct!\n", Color::Green);
            return failures;
        }
        print_error("Try again!");
        failures += 1;
    }
}
