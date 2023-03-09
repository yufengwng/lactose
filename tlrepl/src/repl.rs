use rustyline as rl;
use rustyline::error::ReadlineError;

use tlmito::value::Value;
use tlmito::vm::MitoEnv;
use tlmito::vm::MitoRes;
use tlmito::vm::MitoVM;

pub fn start() -> Result<(), String> {
    Repl::new().start()
}

pub fn run(source: &str) -> Result<Value, String> {
    Repl::new().run(source)
}

struct Repl {
    vm: MitoVM,
    env: MitoEnv,
    editor: rl::Editor<()>,
}

impl Repl {
    pub fn new() -> Self {
        let cfg = rl::Config::builder().edit_mode(rl::EditMode::Vi).build();
        let editor = rl::Editor::<()>::with_config(cfg);
        Self {
            vm: MitoVM::new(),
            env: MitoEnv::with_builtins(),
            editor,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        println!("[[ tile-lang ]]");
        loop {
            let src = self.read_input()?;
            let src = match src {
                Some(s) => s,
                None => return Ok(()),
            };
            if src.is_empty() {
                continue;
            }
            match self.run(&src) {
                Ok(val) => {
                    println!("{}", val);
                    self.env.set("_", val);
                }
                Err(msg) => eprintln!("[E] {}", msg),
            }
        }
    }

    pub fn run(&mut self, source: &str) -> Result<Value, String> {
        match self.vm.run(&mut self.env, source) {
            MitoRes::Ok(val) => Ok(val),
            MitoRes::CompileErr(msg) => Err(format!("compile error: {}", msg)),
            MitoRes::RuntimeErr(msg) => Err(format!("runtime error: {}", msg)),
        }
    }

    fn read_input(&mut self) -> Result<Option<String>, String> {
        let line = self.read_line(">> ")?;
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
            let line = self.read_line("·· ")?; // middot
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
}
