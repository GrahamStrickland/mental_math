// Copyright (C) 2026 Graham Strickland
//
// This file is part of MentalMath.
//
// MentalMath is free software: you can redistribute it and/or modify it under the
// terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option) any
// later version.
//
// MentalMath is distributed in the hope that it will be useful, but WITHOUT ANY
// WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR
// A PARTICULAR PURPOSE. See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License
// along with MentalMath. If not, see <https://www.gnu.org/licenses/>.

use std::convert::{TryFrom, TryInto};
use std::io::{self, Write};

use chrono::{Datelike, NaiveDate, TimeDelta, Weekday};
use colored::Colorize;
use rand::Rng;

use mentalmath::{
    Choices, ExerciseStats, MAX_ADDITIONS, MAX_BIG_NUMBER, MAX_MED_NUMBER, MAX_SMALL_NUMBER, help,
    print_header, random_date_in_range, read_input, timefunc,
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
    print_header("MentalMath");

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
        println!("\n{}", "Choose an exercise:".bold());

        for i in 0..(&stats).len() {
            println!("{} = {}", i + 1, stats[i].name);
        }

        let choice = read_input("Enter selection");
        let choice_trimmed = choice.trim();

        if choice_trimmed.is_empty() {
            continue;
        }

        let choice: usize = match choice_trimmed.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("{}", "Please type a number!".red());
                continue;
            }
        };

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
            Err(_) => eprintln!(
                "{}",
                "Unknown option, please select a number between 1 and 5.".red()
            ),
        }

        if choice > 0 && choice <= stats.len() {
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
}

fn fast_arithmetic(rng: &mut rand::rngs::ThreadRng) -> u32 {
    let mut failures = 0;
    let first_number;
    let second_number;
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
            let a = rng.random_range(MAX_SMALL_NUMBER..MAX_MED_NUMBER);
            let b = rng.random_range(MAX_SMALL_NUMBER..MAX_MED_NUMBER);
            first_number = a.max(b);
            second_number = a.min(b);
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
            second_number = rng.random_range(2..MAX_SMALL_NUMBER);
            let min_quotient = (MAX_SMALL_NUMBER + second_number - 1) / second_number;
            let max_quotient = (MAX_MED_NUMBER - 1) / second_number;
            first_number = rng.random_range(min_quotient..=max_quotient) * second_number;
            expected = first_number / second_number;
            arithmetic_string = format!("{} ÷ {}", first_number, second_number);
        }
        Err(_) => {
            eprintln!(
                "{}",
                "Unknown operation encountered, unable to proceed.".red()
            );
            return failures;
        }
    }

    arithmetic_string.push('\n');

    loop {
        print!("{}", arithmetic_string.blue().bold());
        io::stdout().flush().unwrap();

        let answer = read_input("Enter the answer");

        let answer: i32 = match answer.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                if answer.trim() == "help" || answer.trim() == "--help" || answer.trim() == "-h" {
                    help(Choices::FastArithmetic);
                }
                continue;
            }
        };

        print!("{}", format!("Your answer: {}\n", answer).yellow());

        if answer == expected {
            println!("{}", "You are correct!\n".green().bold());
            return failures;
        }
        eprintln!("{}", "Try again!".red());
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

    loop {
        print!("{}", addition_string.blue().bold());
        io::stdout().flush().unwrap();

        let answer = read_input("Enter the answer");

        let answer: i32 = match answer.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                if answer.trim() == "help" || answer.trim() == "--help" || answer.trim() == "-h" {
                    help(Choices::BasicAddition);
                }
                continue;
            }
        };

        print!("{}", format!("Your answer: {}\n", answer).yellow());

        if answer == terms.iter().sum() {
            println!("{}", "You are correct!\n".green().bold());
            return failures;
        }
        eprintln!("{}", "Try again!".red());
        failures += 1;
    }
}

fn basic_multiplication(rng: &mut rand::rngs::ThreadRng) -> u32 {
    let mut failures = 0;

    let first = rng.random_range(2..MAX_SMALL_NUMBER);
    let second = rng.random_range(MAX_SMALL_NUMBER..=MAX_SMALL_NUMBER);

    loop {
        print!("{}", format!("{} x {}\n", first, second).blue().bold());
        io::stdout().flush().unwrap();

        let answer = read_input("Enter the answer");

        let answer: i32 = match answer.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                if answer.trim() == "help" || answer.trim() == "--help" || answer.trim() == "-h" {
                    help(Choices::BasicMultiplication);
                }
                continue;
            }
        };

        print!("{}", format!("Your answer: {}\n", answer).yellow());

        if answer == first * second {
            println!("{}", "You are correct!\n".green().bold());
            return failures;
        }
        eprintln!("{}", "Try again!".red());
        failures += 1;
    }
}

fn cross_multiplication(rng: &mut rand::rngs::ThreadRng) -> u32 {
    let mut failures = 0;

    let big_number = rng.random_range(MAX_SMALL_NUMBER..MAX_BIG_NUMBER);
    let small_number = rng.random_range(2..MAX_SMALL_NUMBER);

    loop {
        print!(
            "{}",
            format!("{} x {}\n", big_number, small_number).blue().bold()
        );
        io::stdout().flush().unwrap();

        let answer = read_input("Enter the answer");

        let answer: i32 = match answer.trim().parse() {
            Ok(i) => i,
            Err(_) => {
                if answer.trim() == "help" || answer.trim() == "--help" || answer.trim() == "-h" {
                    help(Choices::CrossMultiplication);
                }
                continue;
            }
        };

        print!("{}", format!("Your answer: {}\n", answer).yellow());

        if answer == big_number * small_number {
            println!("{}", "You are correct!\n".green().bold());
            return failures;
        }
        eprintln!("{}", "Try again!".red());
        failures += 1;
    }
}

fn calculate_calendar_dates(rng: &mut rand::rngs::ThreadRng) -> u32 {
    let mut failures = 0;

    let rand_date = random_date_in_range(
        rng,
        NaiveDate::from_ymd_opt(1600, 1, 1).unwrap(),
        NaiveDate::from_ymd_opt(2099, 12, 31).unwrap(),
    );

    loop {
        print!("{}", format!("Date: {}\n", rand_date).blue().bold());
        io::stdout().flush().unwrap();

        let answer = read_input("Enter the day of the week");

        let weekday = match answer.trim().parse::<Weekday>() {
            Ok(weekday) => weekday,
            Err(_) => {
                if answer.trim() == "help" || answer.trim() == "--help" || answer.trim() == "-h" {
                    help(Choices::CalendarDates);
                }
                continue;
            }
        };

        print!("{}", format!("Your answer: {}\n", weekday).yellow());

        if weekday == rand_date.weekday() {
            println!("{}", "You are correct!\n".green().bold());
            return failures;
        }
        eprintln!("{}", "Try again!".red());
        failures += 1;
    }
}
