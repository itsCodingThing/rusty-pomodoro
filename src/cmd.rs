use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "hcmd", version = "2.0.0", about = "enhanced cmds", long_about = None)]
pub struct Cmd {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand, Clone)]
pub enum Commands {
    Pomo {
        #[command(subcommand)]
        command: PomoCommands,
    },
    Fd,
}

#[derive(Debug, Subcommand, Clone)]
pub enum PomoCommands {
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
    },

    #[command(arg_required_else_help = true)]
    #[command(name = "remove", about = "remove a stored timer by name or id")]
    Remove {
        /// timer a name
        #[arg(short, long)]
        name: Option<String>,
    },

    #[command(name = "nuke", about = "remove all stored timers")]
    Nuke,

    #[command(name = "list", about = "list all the availalbe timers")]
    List,
}

pub fn create() -> Cmd {
    Cmd::parse()
}

impl Cmd {
    pub fn commands(&self) -> Commands {
        self.command.clone()
    }
}
