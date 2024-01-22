fn main() {
    if let Err(e) = catfish::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
