mod repl;

fn main() {
    println!("[lactose-lang]");
    match repl::start() {
        Ok(_) => (),
        Err(e) => eprintln!("[E] {}", e),
    }
}
