use clap::Parser;
use clap::Subcommand;

use ltrepl::repl;

#[derive(Parser)]
#[clap(version)]
#[clap(about = "the lactose language")]
struct Cli {
    #[clap(subcommand)]
    cmd: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Start the interactive command line prompt
    Repl,
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Some(cmd) => handle(cmd),
        None => handle_default(),
    }
}

fn handle(cmd: Command) {
    match cmd {
        Command::Repl => run_repl(),
    }
}

fn handle_default() {
    run_repl();
}

fn run_repl() {
    println!("[lactose-lang]");
    match repl::start() {
        Ok(_) => (),
        Err(e) => eprintln!("[E] {}", e),
    }
}
