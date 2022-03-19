use rustyline::error::ReadlineError;
use rustyline::Editor;

use ltmito::vm::MitoEnv;
use ltmito::vm::MitoRes;
use ltmito::vm::MitoVM;

pub fn start() -> Result<(), String> {
    let mut vm = MitoVM::new();
    let mut env = MitoEnv::new();
    let mut editor = Editor::<()>::new();
    loop {
        let mut lines = Vec::new();
        let mut input = editor.readline("lt> ");
        loop {
            // let len = io::stdin().read_line(&mut buffer)?;
            // if len == 0 {
            //     // Reached EOF, erase control char by writing backspace and exit.
            //     println!("{}", 8_u8 as char);
            //     return Ok(());
            // }

            let line = match input {
                Ok(line) => line,
                Err(ReadlineError::Interrupted) => return Ok(()),
                Err(ReadlineError::Eof) => return Ok(()),
                Err(err) => {
                    return Err(format!("error reading input: {}", err));
                }
            };

            let line = line.trim();
            if line.ends_with(";;") {
                let line = &line[..line.len() - 2];
                lines.push(line.to_owned());
                break;
            }
            if line.is_empty() && lines.is_empty() {
                break;
            }
            lines.push(line.to_owned());

            input = editor.readline("  · "); // middot
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
            MitoRes::CompileErr(msg) => return Err(format!("compile error: {}", msg)),
            MitoRes::RuntimeErr(msg) => return Err(format!("runtime error: {}", msg)),
        }
    }
}
