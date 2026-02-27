use crate::{
    cmd::{self, Commands},
    pomo,
};

pub fn init() {
    let cmd = cmd::create();

    match cmd.commands() {
        Commands::Pomo { command } => pomo::pomo(command),

        Commands::Fd => {
            todo!("implement")
        }
    }
}
