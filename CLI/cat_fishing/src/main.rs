fn main() {
    if let Err(e) = cat_fishing::get_args().and_then(cat_fishing::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
