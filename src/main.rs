use std::convert::{TryFrom, TryInto};
use std::io;

use chrono::{Datelike, NaiveDate, TimeDelta, Weekday};
use rand::Rng;
use termcolor::Color;

use mental_math::{
    Choices, ExerciseStats, MAX_ADDITIONS, MAX_BIG_NUMBER, MAX_MED_NUMBER, MAX_SMALL_NUMBER, help,
    print_color, print_error, random_date_in_range, timefunc,
};

enum Operations {
    Addition = 1,
    Subtraction,
    Multiplication,
    Division,
}

impl TryFrom<usize> for Operations {
    type Error = ();

    fn try_from(i: usize) -> Result<Self, Self::Error> {
        match i {
            x if x == Operations::Addition as usize => Ok(Operations::Addition),
            x if x == Operations::Subtraction as usize => Ok(Operations::Subtraction),
            x if x == Operations::Multiplication as usize => Ok(Operations::Multiplication),
            x if x == Operations::Division as usize => Ok(Operations::Division),
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

    let mut stats = vec![
        ExerciseStats::new("Fast Arithmetic"),
        ExerciseStats::new("Basic Addition"),
        ExerciseStats::new("Basic Multiplication"),
        ExerciseStats::new("Cross Multiplication"),
        ExerciseStats::new("Calculate Calendar Dates"),
    ];

    let mut rng = rand::rng();
    let mut failures = 0;
    let mut duration = TimeDelta::zero();

    loop {
        println!("Choose an exercise:");

        for i in 0..(&stats).len() {
            println!("{} = {}", i + 1, stats[i].name);
        }

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: usize = choice.trim().parse().expect("Please type a number!");

        match choice.try_into() {
            Ok(Choices::FastArithmetic) => {
                (failures, duration) = timefunc(fast_arithmetic, &mut rng);
            }
            Ok(Choices::BasicAddition) => {
                (failures, duration) = timefunc(basic_addition, &mut rng);
            }
            Ok(Choices::BasicMultiplication) => {
                (failures, duration) = timefunc(basic_multiplication, &mut rng);
            }
            Ok(Choices::CrossMultiplication) => {
                (failures, duration) = timefunc(cross_multiplication, &mut rng);
            }
            Ok(Choices::CalendarDates) => {
                (failures, duration) = timefunc(calculate_calendar_dates, &mut rng);
            }
            Err(_) => print_error("Unknown option, please select a number between 1 and 4."),
        }
        stats[choice - 1].failures += failures;
        stats[choice - 1].successes += 1;
        stats[choice - 1].duration += duration;

        for ex in &stats {
            if ex.successes > 0 {
                ex.print();
            }
        }
    }
}

fn fast_arithmetic(rng: &mut rand::rngs::ThreadRng) -> u32 {
    let mut failures = 0;
    let first_number;
    let mut second_number;
    let expected;
    let op: usize = rng.random_range(1..=4);
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
                    help(Choices::FastArithmetic);
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
                    help(Choices::BasicAddition);
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
                    help(Choices::BasicMultiplication);
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
                    help(Choices::CrossMultiplication);
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
                    help(Choices::CalendarDates);
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
