fn main() {
    if let Err(e) = head_ache::get_args().and_then(head_ache::run) {
        eprintln!("{}", e);
        std::process::exit(1)
    }
}
