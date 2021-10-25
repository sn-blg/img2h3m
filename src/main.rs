use clap::{App, Arg};

fn main() {
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
            Arg::with_name("subterranean")
                .short("s")
                .help("Update subterranean map if sets"),
        )
        .get_matches();

    let image = matches.value_of("image").unwrap();
    println!("Value for image: {}", image);

    let map = matches.value_of("map").unwrap();
    println!("Value for map: {}", map);

    if matches.is_present("subterranean") {
        println!("subterranean");
    } else {
        println!("default");
    }
}
