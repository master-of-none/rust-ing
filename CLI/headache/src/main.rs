fn main() {
    if let Err(e) = headache::get_args().and_then(headache::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
