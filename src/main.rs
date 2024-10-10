use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;
use std::u64;
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

    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let file = File::open("alarm.mp3").unwrap();
    let sound_duration: u64 = 6;

    let reader = BufReader::new(file);
    // Decode that sound file into a source
    let source = Decoder::new(reader).unwrap();

    let args = Args::parse();
    let min = args.create;
    println!("Mins: {}", min);

    let total_seconds = min * 60;
    let fifty_pers_seconds = total_seconds / 2;

    let bar = ProgressBar::new(total_seconds);
    let default_style = ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/green}")
        .unwrap()
        .progress_chars("#>-");
    let warn_style = ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/yellow}")
        .unwrap()
        .progress_chars("#>-");

    bar.set_style(default_style);

    let mut timer = time::interval(Duration::from_secs(1));
    let mut seconds = Duration::from_secs(total_seconds);
    loop {
        timer.tick().await;
        if seconds.is_zero() {
            break;
        }

        if seconds == Duration::from_secs(fifty_pers_seconds) {
            println!("50% of time is completed.");
            bar.set_style(warn_style);
        }

        seconds = seconds.saturating_sub(Duration::from_secs(1));
        bar.inc(1);
    }

    bar.finish();
    println!("Time is completed.");

    // Play the sound directly on the device
    match stream_handle.play_raw(source.convert_samples()) {
        Ok(_) => {
            // The sound plays in a separate audio thread,
            // so we need to keep the main thread alive while it's playing.
            time::sleep(Duration::from_secs(sound_duration)).await;
        }
        Err(_) => {
            println!("something went wrong and unable to play audio");
        }
    }
}
