use chrono::{Duration, Local, NaiveDate, TimeDelta};
use rand::Rng;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
        Choices::FastArithmetic => println!(
            "\
================
Fast Arithmetic:
================
No help here!
"
        ),
        Choices::BasicAddition => println!(
            "\
===============
Basic Addition:
===============
No help here!
"
        ),
        Choices::BasicMultiplication => {
            print!(
                "\
=====================
Basic Multiplication:
=====================
"
            );
            for i in 1..=MAX_SMALL_NUMBER {
                for j in 1..=MAX_SMALL_NUMBER {
                    print!("{:>4} ", i * j);
                }
                println!("\n");
            }
        }
        Choices::CrossMultiplication => {
            print!(
                "\
=====================
Cross Multiplication:
=====================
Basic Method for Small Numbers:
When calculating a multiplication where one of the numbers is small, such as "
            );
            print_color("68435", Color::Green);

            print_color(
                " × 18,\nit may be fastest to simply add together multiples of the smaller number:\n\n",
                Color::White,
            );

            print_color("         5", Color::Green);
            print_color(" × 18 =  ", Color::White);
            print_color("9", Color::Red);
            print_color("0", Color::Blue);
            print_color(" ⇒ ......", Color::White);
            print_color("0\n", Color::Blue);

            print_color("     9", Color::Red);
            print_color(" + ", Color::White);
            print_color("3", Color::Green);
            print_color(" × 18 =  ", Color::White);
            print_color("6", Color::Red);
            print_color("3", Color::Blue);
            print_color(" ⇒ .....", Color::White);
            print_color("3", Color::Blue);
            print_color("0\n", Color::White);

            print_color("     6", Color::Red);
            print_color(" + ", Color::White);
            print_color("4", Color::Green);
            print_color(" × 18 =  ", Color::White);
            print_color("7", Color::Red);
            print_color("8", Color::Blue);
            print_color(" ⇒ ....", Color::White);
            print_color("8", Color::Blue);
            print_color("30\n", Color::White);

            print_color("     7", Color::Red);
            print_color(" + ", Color::White);
            print_color("8", Color::Green);
            print_color(" × 18 = ", Color::White);
            print_color("15", Color::Red);
            print_color("1", Color::Blue);
            print_color(" ⇒ ...", Color::White);
            print_color("1", Color::Blue);
            print_color("830\n", Color::White);

            print_color("    15", Color::Red);
            print_color(" + ", Color::White);
            print_color("6", Color::Green);
            print_color(" × 18 = ", Color::White);
            print_color("123", Color::Blue);
            print_color(" ⇒ ", Color::White);
            print_color("123", Color::Blue);
            print_color("1830\n\n", Color::White);
        }
        Choices::CalendarDates => {
            println!(
                "\
===========================
Calculating Calendar Dates:
===========================
"
            );

            print_color(
                "\
Century:
    1600s / 2000s / 2400s /…  +2
    1700s / 2100s / 2500s /…  +0
    1800s / 2200s / 2600s /…  +5
    1900s / 2300s / 2700s /…  +3

",
                Color::Red,
            );

            print_color("\
Year:
    Divide the year by 4 and ignore any remainder. Then add this to the original year. Find the remainder when dividing by 7.

", Color::Yellow);

            print_color(
                "\
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

",
                Color::Green,
            );

            print_color("\
Day:
    Just use the date itself. But to simplify calculation later, it is better to find the remainder when dividing by 7.

", Color::Blue);

            print_color("\
Add together the 4 contributions, but if the date is in January or February in a leap year, you must subtract one.

", Color::White);
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
        println!("{}", "=".repeat(self.name.len() + 1));
        println!("{}", self.name);
        println!("{}", "=".repeat(self.name.len() + 1));
        println!(
            "\
Total played = {}
Attempts = {}
Success rate = {:.2}%
Average speed = {:.3}s
        ",
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

pub fn print_color(string: &str, color: Color) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    match stdout.set_color(ColorSpec::new().set_fg(Some(color))) {
        Ok(_) => print!("{}", string),
        Err(e) => eprintln!("{}", e),
    }

    match stdout.set_color(ColorSpec::new().set_fg(Some(Color::White))) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}

pub fn print_error(err_string: &str) {
    let mut stderr = StandardStream::stderr(ColorChoice::Always);

    match stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))) {
        Ok(_) => eprintln!("{}", err_string),
        Err(e) => eprintln!("{}", e),
    }

    match stderr.set_color(ColorSpec::new().set_fg(Some(Color::White))) {
        Ok(_) => {}
        Err(e) => eprintln!("{}", e),
    }
}
