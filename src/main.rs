use std::fs::File;
use std::io::Read;

use clap::Parser;
use clap::Subcommand;

use tbrepl::repl;

const ABOUT: &str = "the table language";
const USAGE: &str = "table <OPTIONS> [FILE | SUBCOMMAND ...]";

#[derive(Parser)]
#[clap(version)]
#[clap(about = ABOUT)]
#[clap(override_usage = USAGE)]
#[clap(disable_colored_help = true)]
struct Cli {
    #[clap(subcommand)]
    cmd: Option<Cmd>,
    /// Script file to run
    file: Option<String>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Start command-line interface (default)
    Cli,
    /// Build program
    Build {
        /// Program file to build
        file: String,
    },
    /// Run script file
    Run {
        /// Script file to run
        file: String,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.cmd {
        Some(cmd) => handle(&cmd),
        None => handle_default(&cli),
    }
}

fn handle_default(cli: &Cli) {
    if let Some(file) = &cli.file {
        run_script(file);
    } else {
        run_repl();
    }
}

fn handle(cmd: &Cmd) {
    match cmd {
        Cmd::Cli => run_repl(),
        Cmd::Build { file } => run_build(file),
        Cmd::Run { file } => run_script(file),
    }
}

fn run_build(_file: &str) {
    println!("work-in-progress");
}

fn run_script(file: &str) {
    let mut file = match File::open(file) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("[E] {}", e);
            return;
        }
    };

    let mut source = String::new();
    if let Err(e) = file.read_to_string(&mut source) {
        eprintln!("[E] {}", e);
        return;
    }

    match repl::run(&source) {
        Ok(_) => {}
        Err(e) => eprintln!("[E] {}", e),
    }
}

fn run_repl() {
    match repl::start() {
        Ok(_) => {}
        Err(e) => eprintln!("[E] {}", e),
    }
}
