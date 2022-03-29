use rustyline::error::ReadlineError;
use rustyline::Editor;

use ltmito::vm::MitoEnv;
use ltmito::vm::MitoRes;
use ltmito::vm::MitoVM;

pub fn start() -> Result<(), String> {
    Repl::new().start()
}

struct Repl {
    vm: MitoVM,
    env: MitoEnv,
    editor: Editor<()>,
}

impl Repl {
    fn new() -> Self {
        Self {
            vm: MitoVM::new(),
            env: MitoEnv::new(),
            editor: Editor::<()>::new(),
        }
    }

    fn start(&mut self) -> Result<(), String> {
        loop {
            let src = self.read_input()?;
            let src = match src {
                Some(s) => s,
                None => return Ok(()),
            };
            if src.is_empty() {
                continue;
            }
            self.run_source(&src);
        }
    }

    fn read_input(&mut self) -> Result<Option<String>, String> {
        let line = self.read_line("lt> ")?;
        let line = match line {
            Some(ln) => ln,
            None => return Ok(None),
        };

        let line = line.trim();
        if !line.ends_with("\\;") {
            return Ok(Some(line.to_owned()));
        }

        let mut lines = Vec::new();
        lines.push(line[..line.len() - 2].to_owned());

        loop {
            let line = self.read_line("  · ")?; // middot
            let line = match line {
                Some(ln) => ln,
                None => return Ok(None),
            };

            let line = line.trim();
            if line.ends_with(";;") {
                lines.push(line[..line.len() - 2].to_owned());
                break;
            }

            lines.push(line.to_owned());
        }

        let input = lines.join("\n").trim().to_owned();
        Ok(Some(input))
    }

    fn read_line(&mut self, prompt: &str) -> Result<Option<String>, String> {
        match self.editor.readline(prompt) {
            Ok(line) => Ok(Some(line)),
            Err(ReadlineError::Interrupted) => Ok(None),
            Err(ReadlineError::Eof) => Ok(None),
            Err(err) => Err(format!("error reading input: {}", err)),
        }
    }

    fn run_source(&mut self, src: &str) {
        match self.vm.run(&mut self.env, src) {
            MitoRes::Ok(val) => {
                println!("{}", val);
                self.env.set("_", val);
            }
            MitoRes::CompileErr(msg) => eprintln!("compile error: {}", msg),
            MitoRes::RuntimeErr(msg) => eprintln!("runtime error: {}", msg),
        }
    }
}
