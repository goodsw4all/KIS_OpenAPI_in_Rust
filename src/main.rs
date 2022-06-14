mod cli;

fn main() {
    if let Err(e) = cli::get_args().and_then(cli::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
