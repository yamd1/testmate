fn main() {
    if let Err(e) = testp::get_args().and_then(testp::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
