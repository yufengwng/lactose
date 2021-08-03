pub struct Aqvm;

impl Aqvm {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&mut self, source: &str) {
        println!("{}", source);
    }
}
