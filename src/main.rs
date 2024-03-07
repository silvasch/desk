fn main() {
    match desk::run() {
        Ok(()) => {}
        Err(e) => eprintln!("{}", e),
    }
}
