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
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

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
        &mut stdout,
    );

    print_color("\
Year:
    Divide the year by 4 and ignore any remainder. Then add this to the original year. Find the remainder when dividing by 7.

", Color::Yellow, &mut stdout);

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
        &mut stdout,
    );

    print_color("\
Day:
    Just use the date itself. But to simplify calculation later, it is better to find the remainder when dividing by 7.

", Color::Blue, &mut stdout);

    print_color("\
Add together the 4 contributions, but if the date is in January or February in a leap year, you must subtract one.

", Color::White, &mut stdout);
}

pub fn cross_multiplication_help() {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    println!(
        "\
=====================
Cross Multiplication:
=====================
Basic Method for Small Numbers:
When calculating a multiplication where one of the numbers is small, such as 68435 × 18, 
it may be fastest to simply add together multiples of the smaller number:
"
    );

    print_color("         5", Color::Green, &mut stdout);
    print_color(" × 18 =  ", Color::White, &mut stdout);
    print_color("9", Color::Red, &mut stdout);
    print_color("0", Color::Blue, &mut stdout);
    print_color(" ⇒ ......", Color::White, &mut stdout);
    print_color("0\n", Color::Blue, &mut stdout);

    print_color("     9", Color::Red, &mut stdout);
    print_color(" + ", Color::White, &mut stdout);
    print_color("3", Color::Green, &mut stdout);
    print_color(" × 18 =  ", Color::White, &mut stdout);
    print_color("6", Color::Red, &mut stdout);
    print_color("3", Color::Blue, &mut stdout);
    print_color(" ⇒ .....", Color::White, &mut stdout);
    print_color("3", Color::Blue, &mut stdout);
    print_color("0\n", Color::White, &mut stdout);

    print_color("     6", Color::Red, &mut stdout);
    print_color(" + ", Color::White, &mut stdout);
    print_color("4", Color::Green, &mut stdout);
    print_color(" × 18 =  ", Color::White, &mut stdout);
    print_color("7", Color::Red, &mut stdout);
    print_color("8", Color::Blue, &mut stdout);
    print_color(" ⇒ ....", Color::White, &mut stdout);
    print_color("8", Color::Blue, &mut stdout);
    print_color("30\n", Color::White, &mut stdout);

    print_color("     7", Color::Red, &mut stdout);
    print_color(" + ", Color::White, &mut stdout);
    print_color("8", Color::Green, &mut stdout);
    print_color(" × 18 = ", Color::White, &mut stdout);
    print_color("15", Color::Red, &mut stdout);
    print_color("1", Color::Blue, &mut stdout);
    print_color(" ⇒ ...", Color::White, &mut stdout);
    print_color("1", Color::Blue, &mut stdout);
    print_color("830\n", Color::White, &mut stdout);

    print_color("    15", Color::Red, &mut stdout);
    print_color(" + ", Color::White, &mut stdout);
    print_color("6", Color::Green, &mut stdout);
    print_color(" × 18 = ", Color::White, &mut stdout);
    print_color("123", Color::Blue, &mut stdout);
    print_color(" ⇒ ", Color::White, &mut stdout);
    print_color("123", Color::Blue, &mut stdout);
    print_color("1830\n\n", Color::White, &mut stdout);
}

fn print_color(string: &str, color: Color, stdout: &mut StandardStream) {
    match stdout.set_color(ColorSpec::new().set_fg(Some(color))) {
        Ok(_) => print!("{}", string),
        Err(e) => eprintln!("{}", e),
    }
}
