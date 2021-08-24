use svg2appicon::*;

use std::path::Path;
use clap::{Arg, App};

fn main() {
    generate_icons(&get_config())
}

fn get_config() -> Config {
    let matches = App::new("svg2xc")
        .version("0.1.0")
        .author("Katsu Matsuda")
        .about("Convert svg to icons for iOS, macOS, and watchOS")
        .arg(Arg::with_name("SVG_PATH")
            .help("Path to svg image")
            .required(true)
            .index(1)
        )
        .arg(Arg::with_name("ASSETS_PATH")
            .help("Path to appiconset")
            .required(true)
            .index(2)
        )
        .arg(Arg::with_name("ios")
            .help("generate icon for iOS")
            .long("ios")
        )
        .arg(Arg::with_name("mac")
            .help("generate icon for macOS")
            .long("mac")
        )
        .arg(Arg::with_name("watch")
            .help("generate icon for watchOS")
            .long("watch")
        )
        .get_matches();

    let svg_path = Path::new(&matches.value_of("SVG_PATH").unwrap().to_string()).to_owned();
    let assets_path = matches.value_of("ASSETS_PATH").unwrap().to_string();
    let mut ios = matches.is_present("ios");
    let mac = matches.is_present("mac");
    let watch = matches.is_present("watch");

    if !ios && !mac && !watch {
        ios = true;
    }

    Config { svg: SVG::File(svg_path), assets_path, ios, mac, watch }
}
