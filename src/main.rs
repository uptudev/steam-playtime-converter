use clap::builder::styling::{AnsiColor, Effects, Styles};
use clap::Parser;

/* Argument parsing */

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .literal(AnsiColor::BrightBlue.on_default())
        .placeholder(AnsiColor::Green.on_default())
}

fn gen_help_template() -> String {
    const VER_LEN: usize = env!("CARGO_PKG_VERSION").len() - 5;
    const BASE: &'static str = "\
\x1b[0;90m┍━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┑
│\x1b[1;4;96m{name} v{version}\x1b[0m {author}\x1b[0;90m│
┕━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┙\x1b[0m
{before-help}
{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
";

    let mut out = BASE.to_string();
    let mut i = VER_LEN;

    while i > 0 {
        out.insert(103 + (VER_LEN - i) * 3, '━');
        out.insert(24, '━');
        i -= 1;
    }

    out
}

const ABOUT: &'static str = "\x1b[0;1;95;4msteam_runtime_converter\x1b[0m is a simple CLI tool which converts Steam's runtime in hours to seconds, minutes, days, months, and years.";
const AUTHOR: &'static str = "by \x1b[0;93muptu\x1b[0m <\x1b[33muptu@uptu.dev\x1b[0m>";

#[derive(Parser, Debug)]
#[command(
    version,                    // enables `-v` and `--version` to print the version
    about = ABOUT,
    author = AUTHOR,
    styles = styles(),
    long_about = None,          // the long version of `-h`, `--help` has no special text
    next_line_help = true,      // option help info is given on a new line
    help_template = gen_help_template()
)]
struct Args {
    /// Number of hours to convert
    #[arg(
        name = "HOURS",
        short = 'H', 
        long = "hours", 
        verbatim_doc_comment
    )]
    hours: Option<f64>,
}



fn main() {
    let args = Args::parse();
    println!("{}", convert_hours(args.hours.unwrap_or_else (|| {
        let mut input = String::new();
        println!("Enter the number of hours to convert:");
        std::io::stdin().read_line(&mut input).unwrap();
        input.trim().parse().ok()
        }.expect("No input provided"))  
    ));
}

fn convert_hours(hours: f64) -> String {
    let seconds = hours * 3600.;
    let minutes = hours * 60.;
    let days = hours / 24.;
    let months = days / 30.;
    let years = months / 12.;
    format!("\x1b[1mInput hours\x1b[0m: {hours}
        Seconds: {seconds}
        Minutes: {minutes}
        Days: {days}
        Months: {months}
        Years: {years}")
}
