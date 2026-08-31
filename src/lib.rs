use chrono::{Duration, Local, NaiveDate, TimeDelta};
use colored::Colorize;
use rand::Rng;

pub static MAX_ADDITIONS: i32 = 5;
pub static MAX_BIG_NUMBER: i32 = 99999;
pub static MAX_MED_NUMBER: i32 = 999;
pub static MAX_SMALL_NUMBER: i32 = 20;

pub enum Choices {
    FastArithmetic = 1,
    BasicAddition,
    BasicMultiplication,
    CrossMultiplication,
    CalendarDates,
}

impl TryFrom<usize> for Choices {
    type Error = ();

    fn try_from(i: usize) -> Result<Self, Self::Error> {
        match i {
            x if x == Choices::FastArithmetic as usize => Ok(Choices::FastArithmetic),
            x if x == Choices::BasicAddition as usize => Ok(Choices::BasicAddition),
            x if x == Choices::BasicMultiplication as usize => Ok(Choices::BasicMultiplication),
            x if x == Choices::CrossMultiplication as usize => Ok(Choices::CrossMultiplication),
            x if x == Choices::CalendarDates as usize => Ok(Choices::CalendarDates),
            _ => Err(()),
        }
    }
}

pub fn help(choice: Choices) {
    match choice {
        Choices::FastArithmetic => {
            print_header("Fast Arithmetic");
            println!("No help here!\n");
        }
        Choices::BasicAddition => {
            print_header("Basic Addition");
            println!("No help here!\n");
        }
        Choices::BasicMultiplication => {
            print_header("Basic Multiplication");
            for i in 1..=MAX_SMALL_NUMBER {
                for j in 1..=MAX_SMALL_NUMBER {
                    print!("{:>4} ", i * j);
                }
                println!("\n");
            }
        }
        Choices::CrossMultiplication => {
            print_header("Cross Multiplication");
            print!(
                "Basic Method for Small Numbers:\n\
                 When calculating a multiplication where one of the numbers is small, such as "
            );
            print!("{}", "68435".green());

            print!(
                "{}",
                " × 18,\nit may be fastest to simply add together multiples of the smaller number:\n\n".white()
            );

            print!("{}", "         5".green());
            print!("{}", " × 18 =  ".white());
            print!("{}", "9".red());
            print!("{}", "0".blue());
            print!("{}", " ⇒ ......".white());
            println!("{}", "0".blue());

            print!("{}", "     9".red());
            print!("{}", " + ".white());
            print!("{}", "3".green());
            print!("{}", " × 18 =  ".white());
            print!("{}", "6".red());
            print!("{}", "3".blue());
            print!("{}", " ⇒ .....".white());
            print!("{}", "3".blue());
            println!("{}", "0".white());

            print!("{}", "     6".red());
            print!("{}", " + ".white());
            print!("{}", "4".green());
            print!("{}", " × 18 =  ".white());
            print!("{}", "7".red());
            print!("{}", "8".blue());
            print!("{}", " ⇒ ....".white());
            print!("{}", "8".blue());
            println!("{}", "30".white());

            print!("{}", "     7".red());
            print!("{}", " + ".white());
            print!("{}", "8".green());
            print!("{}", " × 18 = ".white());
            print!("{}", "15".red());
            print!("{}", "1".blue());
            print!("{}", " ⇒ ...".white());
            print!("{}", "1".blue());
            println!("{}", "830".white());

            print!("{}", "    15".red());
            print!("{}", " + ".white());
            print!("{}", "6".green());
            print!("{}", " × 18 = ".white());
            print!("{}", "123".blue());
            print!("{}", " ⇒ ".white());
            print!("{}", "123".blue());
            println!("{}", "1830\n".white());
        }
        Choices::CalendarDates => {
            print_header("Calculating Calendar Dates");

            println!("{}",
                "Century:\n    1600s / 2000s / 2400s /…  +2\n    1700s / 2100s / 2500s /…  +0\n    1800s / 2200s / 2600s /…  +5\n    1900s / 2300s / 2700s /…  +3\n".red()
            );

            println!("{}", "Year:\n    Divide the year by 4 and ignore any remainder. Then add this to the original year. Find the remainder when dividing by 7.\n".yellow());

            println!("{}", 
                "Month:\n    January:   +4\n    February:  +0\n    March:     +0\n    April:     +3\n    May:       +5\n    June:      +1\n    July:      +3\n    August:    +6\n    September: +2\n    October:   +4\n    November:  +0\n    December:  +2\n".green()
            );

            println!("{}", "Day:\n    Just use the date itself. But to simplify calculation later, it is better to find the remainder when dividing by 7.\n".blue());

            println!("{}", "Add together the 4 contributions, but if the date is in January or February in a leap year, you must subtract one.\n".white());
        }
    }
}

pub struct ExerciseStats {
    pub name: &'static str,
    pub successes: u32,
    pub failures: u32,
    pub duration: TimeDelta,
}

impl ExerciseStats {
    pub fn new(name: &'static str) -> ExerciseStats {
        ExerciseStats {
            name: name,
            successes: 0,
            failures: 0,
            duration: TimeDelta::zero(),
        }
    }

    pub fn print(self: &ExerciseStats) {
        let attempts = self.successes + self.failures;
        print_header(self.name);
        println!(
            "Total played:  {}\nAttempts:      {}\nSuccess rate:  {:.2}%\nAverage speed: {:.3}s\n",
            self.successes,
            attempts,
            (f64::from(self.successes) / f64::from(attempts)) * 100.0,
            self.duration.as_seconds_f64() / f64::from(self.successes)
        );
    }
}

pub fn timefunc<F: Fn(S) -> T, S, T>(f: F, s: S) -> (T, TimeDelta) {
    let start = Local::now();
    let result = f(s);
    let end = Local::now();
    let duration = end - start;

    (result, duration)
}

pub fn random_date_in_range(
    rng: &mut rand::rngs::ThreadRng,
    start: NaiveDate,
    end: NaiveDate,
) -> NaiveDate {
    let days_in_range = (end - start).num_days();
    let random_days: i64 = rng.random_range(0..days_in_range);
    start + Duration::days(random_days)
}

pub fn print_header(title: &str) {
    let len = title.len();
    let line = "─".repeat(len + 2);
    println!("{}", format!(" ╭{}╮", line).cyan().bold());
    println!(
        " {} {} {}",
        "│".cyan().bold(),
        title.bold(),
        "│".cyan().bold()
    );
    println!("{}", format!(" ╰{}╯", line).cyan().bold());
}

pub fn read_input(prompt: &str) -> String {
    use std::io::{self, Write};

    println!(
        "{}",
        "╭──────────────────────────────────────────────╮".cyan()
    );
    println!("{} {:<44} {}", "│".cyan(), prompt.bold(), "│".cyan());
    print!("{} > ", "│".cyan());
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!(
        "{}",
        "╰──────────────────────────────────────────────╯".cyan()
    );

    input
}
