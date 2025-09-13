use chrono::{Duration, NaiveDate};
use rand::Rng;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub fn random_date_in_range(
    rng: &mut rand::rngs::ThreadRng,
    start: NaiveDate,
    end: NaiveDate,
) -> NaiveDate {
    let days_in_range = (end - start).num_days();
    let random_days: i64 = rng.random_range(0..days_in_range);
    start + Duration::days(random_days)
}

pub fn calendar_dates_help() {
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

pub fn cross_multiplication_help() {
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

pub fn print_statistics(
    calendar_successes: u32,
    calendar_failures: u32,
    multiplication_successes: u32,
    multiplication_failures: u32,
) {
    if calendar_successes > 0 {
        println!(
            "\
===========================
Calculating Calendar Dates:
===========================
    Total played = {calendar_successes}
    Attempts = {}
    Success rate = {}%
    ",
            calendar_successes + calendar_failures,
            ((f64::from(calendar_successes) / f64::from(calendar_successes + calendar_failures))
                * 100.0)
                .round()
        );
    }

    if multiplication_successes > 0 {
        println!(
            "\
=====================
Cross Multiplication:
=====================
    Total played = {multiplication_successes}
    Attempts = {}
    Success rate = {}%
    ",
            multiplication_successes + multiplication_failures,
            ((f64::from(multiplication_successes)
                / f64::from(multiplication_successes + multiplication_failures))
                * 100.0)
                .round()
        );
    }
}
