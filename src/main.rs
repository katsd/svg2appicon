use svg2appicon::*;

use std::path::Path;
use clap::{Arg, App};

fn main() {
    generate_icons(&get_config())
}

fn get_config() -> Config {
    let matches = App::new("svg2appicon")
        .version("0.1.0")
        .author("Katsu Matsuda")
        .about("Convert svg to icons for iOS, macOS, and watchOS")
        .arg(Arg::with_name("ASSETS_PATH")
            .help("Path to appiconset")
            .required(true)
            .index(1)
        )
        .arg(Arg::with_name("SVG_IOS")
            .help("Path to svg image for iOS")
            .long("ios")
            .value_name("FILE")
        )
        .arg(Arg::with_name("SVG_MAC")
            .help("Path to svg image for macOS")
            .long("mac")
            .value_name("FILE")
        )
        .arg(Arg::with_name("SVG_WATCH")
            .help("Path to svg image for watchOS")
            .long("watch")
            .value_name("FILE")
        )
        .get_matches();

    let assets_path = matches.value_of("ASSETS_PATH").unwrap().to_string();
    let svg_ios = matches.value_of("SVG_IOS").map(|v| SVG::File(Path::new(v).to_owned()));
    let svg_mac = matches.value_of("SVG_MAC").map(|v| SVG::File(Path::new(v).to_owned()));
    let svg_watch = matches.value_of("SVG_WATCH").map(|v| SVG::File(Path::new(v).to_owned()));

    Config { assets_path, svg_ios, svg_mac, svg_watch }
}
