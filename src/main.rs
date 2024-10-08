use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use tokio::time;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 10)]
    create: u64,
}

#[tokio::main]
async fn main() {
    let welcome_banner = include_str!("../ascii.txt");
    println!("{}", welcome_banner);

    let args = Args::parse();
    let min = args.create;
    println!("Mins: {}", min);

    let total_seconds = min * 60;
    let fifty_pers_seconds = total_seconds / 2;

    let bar = ProgressBar::new(total_seconds);
    bar.set_style(
        ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/green}")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut timer = time::interval(Duration::from_secs(1));
    let mut seconds = Duration::from_secs(total_seconds);
    loop {
        timer.tick().await;
        if seconds.is_zero() {
            break;
        }

        if seconds == Duration::from_secs(fifty_pers_seconds) {
            println!("50% of time is completed.");
            bar.set_style(
                ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/yellow}")
                    .unwrap()
                    .progress_chars("#>-"),
            );
        }

        seconds = seconds.saturating_sub(Duration::from_secs(1));
        bar.inc(1);
    }

    bar.finish();
    println!("Time is completed.");
}
