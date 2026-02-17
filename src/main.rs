mod cli;
mod cmd;
mod storage;
mod timer;

fn main() {
    let welcome_banner = include_str!("../ascii.txt");
    println!("{}", welcome_banner);

    cli::init();
}
