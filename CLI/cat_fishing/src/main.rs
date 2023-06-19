fn main() {
    if let Err(e) = cat_fishing::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
