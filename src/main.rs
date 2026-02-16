use clap::{Parser, Subcommand};
use indicatif::{ProgressBar, ProgressStyle};
use std::thread;
use std::time::Duration;

mod storage;
mod timer;

#[derive(Parser, Debug)]
#[command(name = "Pomo", version = "1.0.0", about = "a pomodoro counter", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(arg_required_else_help = true)]
    #[command(name = "add", about = "add a timer to storage")]
    Add {
        /// Give timer a name
        #[arg(short, long)]
        name: String,

        /// Crate a timer for specified value in mins
        /// minimum value should be 5 min
        #[arg(short, long, default_value_t = 10)]
        duration: u64,
    },

    #[command(arg_required_else_help = true)]
    #[command(name = "create", about = "create a quick timer")]
    Create {
        /// Give timer a name
        #[arg(short, long)]
        name: String,

        /// Crate a timer for specified value in mins
        /// minimum value should be 5 min
        #[arg(short, long)]
        duration: u64,
    },

    #[command(arg_required_else_help = true)]
    #[command(name = "run", about = "run a stored timer by name")]
    Run {
        /// timer a name
        #[arg(short, long)]
        name: Option<String>,

        /// timer a name
        #[arg(short, long)]
        id: Option<String>,
    },

    #[command(name = "list", about = "list all the availalbe timers")]
    List,
}

fn main() {
    let welcome_banner = include_str!("../ascii.txt");
    println!("{}", welcome_banner);

    let cli = Cli::parse();
    let mut store = storage::create().expect("unable to create storage");

    match cli.command {
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

        Commands::Run { name, id } => {
            println!("name: {name:?}\n");

            if let Some(n) = name {
                for timer in store.timers() {
                    if timer.name() == n.as_str() {
                        println!("{timer:?}")
                    }
                }
            }

            if let Some(n) = id {
                for timer in store.timers() {
                    if timer.name() == n {}
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
