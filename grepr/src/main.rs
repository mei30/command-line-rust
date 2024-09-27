fn main() {
    if let Err(e) = grepr::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
