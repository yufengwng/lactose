mod repl;

fn main() {
    println!("[tassel-lang]");
    match repl::start() {
        Ok(_) => (),
        Err(e) => eprintln!("[E] {}", e),
    }
}
