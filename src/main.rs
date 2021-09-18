mod repl;

fn main() {
    println!("[lite-lang]");
    match repl::start() {
        Ok(_) => (),
        Err(e) => eprintln!("[E] {}", e),
    }
}
