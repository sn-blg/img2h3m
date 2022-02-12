use clap::{App, Arg};
use img2h3m::Config;
use std::process;

pub fn get_config() -> Config {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .arg(
            Arg::with_name("land image")
                .short("l")
                .help("Path to the input land image file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("underground image")
                .short("u")
                .help("Path to the input underground image file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("map")
                .short("m")
                .help("Path to the existing h3m file for update it's minimap")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("otw")
                .help("Use one-tile water"),
        )
        .arg(
            Arg::with_name("obstacles")
                .short("o")
                .help("Create obstacles on the map \
                       (attention, this option may delete all the original objects and events on the input map)"),
        )
        .get_matches();

    if !matches.is_present("land image") && !matches.is_present("underground image") {
        eprintln!(
            "You must set the path to at least one surface image (land image or underground image)"
        );
        process::exit(1);
    }

    Config {
        land_image_path: matches.value_of("land image").map(|i| i.to_string()),
        underground_image_path: matches.value_of("underground image").map(|i| i.to_string()),
        map_path: matches.value_of("map").unwrap().to_string(),
        obstacles: matches.is_present("obstacles"),
        one_tile_water: matches.is_present("otw"),
    }
}
