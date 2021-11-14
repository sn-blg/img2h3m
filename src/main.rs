use std::process;

mod cli;

fn main() {
    let config = cli::get_config();
    if let Err(e) = img2h3m::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
