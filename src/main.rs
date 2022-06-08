fn main() {
    if let Err(e) = trade_lib::cli::get_args() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
