use std::convert::{TryFrom, TryInto};
use std::io;

use chrono::{Datelike, NaiveDate, TimeDelta, Weekday};
use rand::Rng;
use termcolor::Color;

use mental_math::{
    basic_addition_help, basic_multiplication_help, calendar_dates_help, cross_multiplication_help,
    fast_arithmetic_help, print_color, print_error, print_statistics, random_date_in_range,
    timefunc,
};

static MAX_ADDITIONS: i32 = 5;
static MAX_BIG_NUMBER: i32 = 99999;
static MAX_MED_NUMBER: i32 = 999;
static MAX_SMALL_NUMBER: i32 = 20;

enum Operations {
    Addition = 1,
    Subtraction,
    Multiplication,
    Division,
}

impl TryFrom<u32> for Operations {
    type Error = ();

    fn try_from(i: u32) -> Result<Self, Self::Error> {
        match i {
            x if x == Operations::Addition as u32 => Ok(Operations::Addition),
            x if x == Operations::Subtraction as u32 => Ok(Operations::Subtraction),
            x if x == Operations::Multiplication as u32 => Ok(Operations::Multiplication),
            x if x == Operations::Division as u32 => Ok(Operations::Division),
            _ => Err(()),
        }
    }
}

enum Choices {
    FastArithmetic = 1,
    BasicAddition,
    BasicMultiplication,
    CrossMultiplication,
    CalendarDates,
}

impl TryFrom<u32> for Choices {
    type Error = ();

    fn try_from(i: u32) -> Result<Self, Self::Error> {
        match i {
            x if x == Choices::FastArithmetic as u32 => Ok(Choices::FastArithmetic),
            x if x == Choices::BasicAddition as u32 => Ok(Choices::BasicAddition),
            x if x == Choices::BasicMultiplication as u32 => Ok(Choices::BasicMultiplication),
            x if x == Choices::CrossMultiplication as u32 => Ok(Choices::CrossMultiplication),
            x if x == Choices::CalendarDates as u32 => Ok(Choices::CalendarDates),
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

    let (mut fast_arithmetic_successes, mut fast_arithmetic_failures, mut fast_arithmetic_duration) =
        (0, 0, TimeDelta::zero());
    let (mut basic_addition_successes, mut basic_addition_failures, mut basic_addition_duration) =
        (0, 0, TimeDelta::zero());
    let (
        mut basic_multiplication_successes,
        mut basic_multiplication_failures,
        mut basic_multiplication_duration,
    ) = (0, 0, TimeDelta::zero());
    let (
        mut cross_multiplication_successes,
        mut cross_multiplication_failures,
        mut cross_multiplication_duration,
    ) = (0, 0, TimeDelta::zero());
    let (mut calendar_successes, mut calendar_failures, mut calendar_duration) =
        (0, 0, TimeDelta::zero());

    loop {
        println!(
            "\
Choose an exercise:
1 = Fast Arithmetic
2 = Basic Addition
3 = Basic Multiplication
4 = Cross Multiplication
5 = Calculate Calendar Dates 
"
        );
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: u32 = choice.trim().parse().expect("Please type a number!");

        let mut rng = rand::rng();

        match choice.try_into() {
            Ok(Choices::FastArithmetic) => {
                let (failures, duration) = timefunc(fast_arithmetic, &mut rng);
                fast_arithmetic_failures += failures;
                fast_arithmetic_successes += 1;
                fast_arithmetic_duration += duration;
            }
            Ok(Choices::BasicAddition) => {
                let (failures, duration) = timefunc(basic_addition, &mut rng);
                basic_addition_failures += failures;
                basic_addition_successes += 1;
                basic_addition_duration += duration;
            }
            Ok(Choices::BasicMultiplication) => {
                let (failures, duration) = timefunc(basic_multiplication, &mut rng);
                basic_multiplication_failures += failures;
                basic_multiplication_successes += 1;
                basic_multiplication_duration += duration;
            }
            Ok(Choices::CrossMultiplication) => {
                let (failures, duration) = timefunc(cross_multiplication, &mut rng);
                cross_multiplication_failures += failures;
                cross_multiplication_successes += 1;
                cross_multiplication_duration += duration;
            }
            Ok(Choices::CalendarDates) => {
                let (failures, duration) = timefunc(calculate_calendar_dates, &mut rng);
                calendar_failures += failures;
                calendar_successes += 1;
                calendar_duration += duration;
            }
            Err(_) => print_error("Unknown option, please select a number between 1 and 4."),
        }

        print_statistics(
            fast_arithmetic_failures,
            fast_arithmetic_successes,
            fast_arithmetic_duration,
            basic_addition_successes,
            basic_addition_failures,
            basic_addition_duration,
            basic_multiplication_successes,
            basic_multiplication_failures,
            basic_multiplication_duration,
            cross_multiplication_successes,
            cross_multiplication_failures,
            cross_multiplication_duration,
            calendar_successes,
            calendar_failures,
            calendar_duration,
        );
    }
}

fn fast_arithmetic(rng: &mut rand::rngs::ThreadRng) -> u32 {
    let mut failures = 0;
    let first_number;
    let mut second_number;
    let expected;
    let op: u32 = rng.random_range(1..=4);
    let mut arithmetic_string;

    match op.try_into() {
        Ok(Operations::Addition) => {
            first_number = rng.random_range(MAX_SMALL_NUMBER..MAX_MED_NUMBER);
            second_number = rng.random_range(MAX_SMALL_NUMBER..MAX_MED_NUMBER);
            expected = first_number + second_number;
            arithmetic_string = format!("{} + {}", first_number, second_number);
        }
        Ok(Operations::Subtraction) => {
            first_number = rng.random_range(MAX_SMALL_NUMBER..MAX_MED_NUMBER);
            second_number = rng.random_range(MAX_SMALL_NUMBER..MAX_MED_NUMBER);
            while second_number >= first_number {
                second_number = rng.random_range(MAX_SMALL_NUMBER..MAX_MED_NUMBER);
            }
            expected = first_number - second_number;
            arithmetic_string = format!("{} - {}", first_number, second_number);
        }
        Ok(Operations::Multiplication) => {
            first_number = rng.random_range(MAX_SMALL_NUMBER..MAX_MED_NUMBER);
            second_number = rng.random_range(2..MAX_SMALL_NUMBER);
            expected = first_number * second_number;
            arithmetic_string = format!("{} x {}", first_number, second_number);
        }
        Ok(Operations::Division) => {
            first_number = rng.random_range(MAX_SMALL_NUMBER..MAX_MED_NUMBER);
            second_number = rng.random_range(2..MAX_SMALL_NUMBER);
            while first_number % second_number != 0 {
                second_number = rng.random_range(2..MAX_SMALL_NUMBER);
            }
            expected = first_number / second_number;
            arithmetic_string = format!("{} ÷ {}", first_number, second_number);
        }
        Err(_) => {
            print_error("Unknown operation encountered, unable to proceed.");
            return failures;
        }
    }
    arithmetic_string.push('\n');

    println!("Enter the answer:");

    loop {
        print_color(arithmetic_string.as_str(), Color::Blue);

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let answer: i32 = match answer.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                if answer.trim() == "help" || answer.trim() == "--help" || answer.trim() == "-h" {
                    fast_arithmetic_help();
                }
                continue;
            }
        };

        print_color(format!("Your answer: {answer}\n").as_str(), Color::Yellow);

        if answer == expected {
            print_color("You are correct!\n", Color::Green);
            return failures;
        }
        print_error("Try again!");
        failures += 1;
    }
}

fn basic_addition(rng: &mut rand::rngs::ThreadRng) -> u32 {
    let mut failures = 0;
    let terms: Vec<i32> = (1..=rng.random_range(2..=MAX_ADDITIONS))
        .map(|_| rng.random_range(-MAX_BIG_NUMBER..=MAX_BIG_NUMBER))
        .collect();
    let mut addition_string = format!("{}", terms[0]);
    for term in &terms[1..terms.len()] {
        if *term < 0 {
            addition_string.push_str(format!(" - {}", term.abs()).as_str());
        } else {
            addition_string.push_str(format!(" + {}", term).as_str());
        }
    }
    addition_string.push('\n');

    println!("Enter the answer:");

    loop {
        print_color(addition_string.as_str(), Color::Blue);

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let answer: i32 = match answer.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                if answer.trim() == "help" || answer.trim() == "--help" || answer.trim() == "-h" {
                    basic_addition_help();
                }
                continue;
            }
        };

        print_color(format!("Your answer: {answer}\n").as_str(), Color::Yellow);

        if answer == terms.iter().sum() {
            print_color("You are correct!\n", Color::Green);
            return failures;
        }
        print_error("Try again!");
        failures += 1;
    }
}

fn basic_multiplication(rng: &mut rand::rngs::ThreadRng) -> u32 {
    let mut failures = 0;

    println!("Enter the answer:");

    let first = rng.random_range(2..MAX_SMALL_NUMBER);
    let second = rng.random_range(MAX_SMALL_NUMBER..=MAX_SMALL_NUMBER);

    loop {
        print_color(format!("{} x {}\n", first, second).as_str(), Color::Blue);

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Failed to read line");

        let answer: i32 = match answer.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                if answer.trim() == "help" || answer.trim() == "--help" || answer.trim() == "-h" {
                    basic_multiplication_help(MAX_SMALL_NUMBER);
                }
                continue;
            }
        };

        print_color(format!("Your answer: {answer}\n").as_str(), Color::Yellow);

        if answer == first * second {
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

    let big_number = rng.random_range(MAX_SMALL_NUMBER..MAX_BIG_NUMBER);
    let small_number = rng.random_range(2..MAX_SMALL_NUMBER);

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
