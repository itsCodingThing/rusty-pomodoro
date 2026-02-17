use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

use crate::{
    cmd::{self, Commands},
    storage,
};

pub fn init() {
    let cmd = cmd::create();
    let mut store = storage::create().expect("unable to create storage");

    match cmd.commands() {
        Commands::Create { duration, name } => {
            let secs = duration * 60;

            println!("name: {name}");
            setup_timer(secs);
            println!("Timer is complete")
        }

        Commands::Add { name, duration } => {
            println!("name: {name}");
            println!("duration: {duration}");

            store.add(name, duration);
        }

        Commands::Run { name } => {
            let mut duration = 0;
            let mut tname = String::new();

            if let Some(n) = name {
                println!("search by name: {n}");

                for timer in store.timers() {
                    if timer.name() == n.as_str() {
                        tname = timer.name();
                        duration = timer.duration();
                    }
                }
            }

            let secs = duration * 60;
            if secs > 0 {
                println!("name: {tname}");
                println!("mins: {duration}");

                setup_timer(secs);
            }
        }

        Commands::Remove { name } => {
            if let Some(n) = name {
                println!("search by name: {n:?}");
                let mut remove_idx = 0;

                for (i, timer) in store.timers().iter().enumerate() {
                    if timer.name() == n.as_str() {
                        remove_idx = i;
                    }
                }

                if remove_idx > 0 {
                    store.remove(remove_idx);
                }
            }
        }

        Commands::List => {
            if store.timers().is_empty() {
                println!("there are 0 timers");
            }

            for value in store.timers() {
                println!("{value:?}");
            }
        }
    }
}

fn setup_timer(duration: u64) {
    let bar = ProgressBar::new(duration);
    let default_style = ProgressStyle::with_template("[{elapsed_precise}] {bar:40.cyan/green}")
        .unwrap()
        .progress_chars("#>*");
    bar.set_style(default_style);

    let mut seconds = Duration::from_secs(duration);
    let wait = Duration::from_secs(1);

    loop {
        thread::sleep(wait);

        seconds = seconds.saturating_sub(Duration::from_secs(1));
        bar.inc(1);

        if seconds.is_zero() {
            break;
        }
    }

    bar.finish();
}
