use rustyline as rl;
use rustyline::error::ReadlineError;

use tbmito::value::Value;
use tbmito::vm::MitoEnv;
use tbmito::vm::MitoRes;
use tbmito::vm::MitoVM;

const INTRO: &str = "[[ table-lang ]]";
const PROMPT_LINE: &str = ">> ";
const PROMPT_CONT: &str = "·· "; // middot
const MULTI_START: &str = "\\;";
const MULTI_END: &str = ";;";
const RES_VAR: &str = "_";

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
        let mut env = MitoEnv::with_builtins();
        env.set(RES_VAR, Value::Unit);
        Self {
            vm: MitoVM::new(),
            env,
            editor,
        }
    }

    pub fn start(&mut self) -> Result<(), String> {
        println!("{}", INTRO);
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
                    self.env.set(RES_VAR, val);
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
        let line = self.read_line(PROMPT_LINE)?;
        let line = match line {
            Some(ln) => ln,
            None => return Ok(None),
        };

        let line = line.trim();
        if !line.ends_with(MULTI_START) {
            return Ok(Some(line.to_owned()));
        }

        let mut lines = Vec::new();
        lines.push(line[..line.len() - 2].to_owned());

        loop {
            let line = self.read_line(PROMPT_CONT)?;
            let line = match line {
                Some(ln) => ln,
                None => return Ok(None),
            };

            let line = line.trim();
            if line.ends_with(MULTI_END) {
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
