use clap::{App, Arg};
use img2h3m::Config;
use std::process;

pub fn get_config() -> Config {
    let transparent_color = [0, 0xFF, 0xFF];

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
                .help("Path to the existing h3m file for update it's map")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("base tiles")
                .short("b")
                .help("Use only base tiles. Don't use additional tiles (one-tile water)"),
        )
        .arg(
            Arg::with_name("obstacles")
                .short("o")
                .help("Create obstacles on the map \
                       (attention, this option will delete all the original objects and events on the input map)"),
        )
        .arg(
            Arg::with_name("transparent")
                .short("t")
                .help(
                    &format!("Transparent mode: pixels with color 0x{:02X}{:02X}{:02X} are not processed",
                    transparent_color[0], transparent_color[1], transparent_color[2])),
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
        one_tile_water: !matches.is_present("base tiles"),
        transparent_color: if matches.is_present("transparent") {
            Some(transparent_color)
        } else {
            None
        },
    }
}
