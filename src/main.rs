fn main() {
    match notiz::run() {
        Ok(()) => {}
        Err(e) => eprintln!("{}", e),
    }
}
