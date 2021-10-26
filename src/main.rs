use std::process;
fn main() {
    let config = img2h3m::cli::get_config();

    if let Err(e) = img2h3m::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
