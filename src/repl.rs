use std::io;
use std::io::Write;

use tassel::vm::Aqvm;

pub fn start() -> io::Result<()> {
    let mut vm = Aqvm::new();
    let mut buffer = String::new();
    loop {
        print!("> ");
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
            if !input.ends_with('\\') {
                let line = input.to_owned();
                lines.push(line);
                break;
            }

            let line = input[..input.len()-1].to_owned();
            lines.push(line);

            print!("~ ");
            io::stdout().flush()?;
        }

        let source = lines.join("\n").trim().to_owned();
        if source.is_empty() {
            continue;
        }

        vm.run(&source);
    }
}
