use crate::config::Config;
use clap::{App, Arg};

pub fn get_config() -> Config {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("image")
                .short("i")
                .long("img")
                .help("Sets the path of input image file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("map")
                .short("m")
                .long("map")
                .help("Sets the path of existing h3m file for update it's mini map")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("underground")
                .short("u")
                .help("Update underground map if sets"),
        )
        .arg(
            Arg::with_name("fix")
                .short("f")
                .help("fix errors in result map"),
        )
        .get_matches();

    Config {
        image_path: matches.value_of("image").unwrap().to_string(),
        map_path: matches.value_of("map").unwrap().to_string(),
        underground: matches.is_present("underground"),
        fix: matches.is_present("fix"),
    }
}
