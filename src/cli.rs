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
                .help("Path of input image file")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("map")
                .short("m")
                .long("map")
                .help("Path of existing h3m file for update it's minimap")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("underground")
                .short("u")
                .help("Update underground map"),
        )
        .arg(
            Arg::with_name("fix")
                .short("f")
                .help("Fix errors in result map"),
        )
        .arg(
            Arg::with_name("obstacles")
                .short("o")
                .help("Сreate obstacles on the map. \
                       Attention, this option will delete all the original objects and events on the input map."),
        )
        .get_matches();

    Config {
        image_path: matches.value_of("image").unwrap().to_string(),
        map_path: matches.value_of("map").unwrap().to_string(),
        underground: matches.is_present("underground"),
        fix: matches.is_present("fix"),
        obstacles: matches.is_present("obstacles"),
    }
}
