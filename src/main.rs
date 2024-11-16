use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Crate a timer for specified value in mins
    #[arg(short, long, default_value_t = 10)]
    create: u64,

    /// Give timer a name
    #[arg(short, long, default_value_t = String::from("Pomodoro"))]
    name: String,
}

fn main() {
    let welcome_banner = include_str!("../ascii.txt");
    println!("{}", welcome_banner);

    let args = Args::parse();
    let min = args.create;
    println!("Name: {}", args.name);
    println!("Mins: {}", min);

    let total_seconds = min * 60;
    let fifty_pers_seconds = total_seconds / 2;

    let bar = ProgressBar::new(total_seconds);
    let default_style = ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/green}")
        .unwrap()
        .progress_chars("#>-");
    bar.set_style(default_style);

    let mut seconds = Duration::from_secs(total_seconds);

    let wait = Duration::from_secs(1);
    loop {
        thread::sleep(wait);
        if seconds.is_zero() {
            break;
        }

        if seconds == Duration::from_secs(fifty_pers_seconds) {
            println!("50% of time is completed.");
        }

        seconds = seconds.saturating_sub(Duration::from_secs(1));
        bar.inc(1);
    }

    bar.finish();
    println!("Time is completed.");
}
