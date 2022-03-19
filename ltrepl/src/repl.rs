use std::io;
use std::io::Write;

use ltmito::vm::MitoEnv;
use ltmito::vm::MitoRes;
use ltmito::vm::MitoVM;

pub fn start() -> io::Result<()> {
    let mut vm = MitoVM::new();
    let mut env = MitoEnv::new();
    let mut buffer = String::new();
    loop {
        print!("lt> ");
        io::stdout().flush()?;

        let mut lines = Vec::new();
        loop {
            buffer.clear();
            let len = io::stdin().read_line(&mut buffer)?;
            if len == 0 {
                // Reached EOF, erase control char by writing backspace and exit.
                println!("{}", 8_u8 as char);
                return Ok(());
            }

            let input = buffer.trim();
            if input.ends_with(";;") {
                let line = input[..input.len() - 2].to_owned();
                lines.push(line);
                break;
            }

            let line = input.to_owned();
            if line.is_empty() && lines.is_empty() {
                break;
            }
            lines.push(line);

            print!("  · "); // middot
            io::stdout().flush()?;
        }

        let source = lines.join("\n").trim().to_owned();
        if source.is_empty() {
            continue;
        }

        match vm.run(&mut env, &source) {
            MitoRes::Ok(val) => {
                println!("{}", val);
                env.set("_", val);
            }
            MitoRes::CompileErr(msg) => eprintln!("[E] {}", msg),
            MitoRes::RuntimeErr(msg) => eprintln!("[E] {}", msg),
        }
    }
}
