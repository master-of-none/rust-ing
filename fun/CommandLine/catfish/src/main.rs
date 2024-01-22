fn main() {
    if let Err(e) = catfish::get_args().and_then(catfish::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
