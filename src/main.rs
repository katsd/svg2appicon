use svg2appicon::*;

use std::path::Path;
use clap::{Arg, App};
use anyhow::{Result, bail};

fn main() {
    let config = match get_config() {
        Ok(config) => config,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    match generate_icons(&config) {
        Ok(()) => (),
        Err(e) => eprintln!("Error: {}", e)
    };
}

fn get_config() -> Result<Config> {
    let assets_path_key = "APPICONSET";
    let svg_ios_key = "SVG_IOS";
    let svg_mac_key = "SVG_MAC";
    let svg_watch_key = "SVG_WATCH";

    let matches = App::new("svg2appicon")
        .version("0.1.0")
        .author("Katsu Matsuda")
        .about("Convert svg to icons for iOS, macOS, and watchOS")
        .arg(Arg::with_name(assets_path_key)
            .help("Path to .appiconset (e.g. /path/to/Assets.xcassets/AppIcon.appiconset)")
            .required(true)
            .index(1)
        )
        .arg(Arg::with_name(svg_ios_key)
            .help("Generate icons for iOS")
            .long("ios")
            .value_name("SVG_FILE")
        )
        .arg(Arg::with_name(svg_mac_key)
            .help("Generate icons for macOS")
            .long("mac")
            .value_name("SVG_FILE")
        )
        .arg(Arg::with_name(svg_watch_key)
            .help("Generate icons for watchOS")
            .long("watch")
            .value_name("SVG_FILE")
        )
        .get_matches();

    if !matches.is_present(svg_ios_key) && !matches.is_present(svg_mac_key) && !matches.is_present(svg_watch_key) {
        bail!("Please specify at least one svg file");
    }

    let assets_path = matches.value_of(assets_path_key).unwrap().to_string();
    let svg_ios = matches.value_of(svg_ios_key).map(|v| SVG::File(Path::new(v).to_owned()));
    let svg_mac = matches.value_of(svg_mac_key).map(|v| SVG::File(Path::new(v).to_owned()));
    let svg_watch = matches.value_of(svg_watch_key).map(|v| SVG::File(Path::new(v).to_owned()));

    Ok(Config { assets_path, svg_ios, svg_mac, svg_watch })
}
