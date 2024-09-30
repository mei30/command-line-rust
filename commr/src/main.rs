fn main() {
    if let Err(e) = commr::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }

}
