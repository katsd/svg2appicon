mod icon;

use usvg::{SystemFontDB, Tree};
use std::path::Path;
use std::fs::{self, File};
use std::io::Write;
use clap::{Arg, App};
use anyhow::{Context, Result};
use icon::Icon;

struct Config {
    svg_path: String,
    assets_path: String,
    ios: bool,
    mac: bool,
    watch: bool,
}

fn main() {
    let config = get_config();

    let tree = match get_tree(&config.svg_path) {
        Ok(tree) => tree,
        Err(e) => panic!("{}", e)
    };

    fs::remove_dir_all(&config.assets_path)
        .expect(&format!("Failed to remove {}", config.assets_path));
    fs::create_dir(&config.assets_path)
        .expect(&format!("Failed to create {}", config.assets_path));

    let mut icons_set: Vec<&Vec<Icon>> = vec![];
    if config.ios {
        icons_set.push(&icon::IOS_ICONS);
    }
    if config.mac {
        icons_set.push(&icon::MAC_ICONS);
    }
    if config.watch {
        icons_set.push(&icon::WATCH_ICONS);
    }
    let icons_set = icons_set;

    let assets_path_str: &str = &config.assets_path;
    let assets_path = Path::new(assets_path_str);

    for icons in &icons_set {
        match generate_icon_files(assets_path, &tree, &icons) {
            Ok(()) => (),
            Err(e) => panic!("{}", e)
        };
    }

    let json_str = get_json_str(&icons_set);

    match save_json(&assets_path, &json_str) {
        Ok(()) => (),
        Err(e) => panic!("{}", e)
    };
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

    let svg_path = matches.value_of("SVG_PATH").unwrap().to_string();
    let assets_path = matches.value_of("ASSETS_PATH").unwrap().to_string();
    let ios = matches.is_present("ios");
    let mac = matches.is_present("mac");
    let watch = matches.is_present("watch");

    Config { svg_path, assets_path, ios, mac, watch }
}

fn get_json_str(icons_set: &Vec<&Vec<Icon>>) -> String {
    let icons_json_str =
        icons_set
            .iter()
            .map(|icons|
                icons
                    .iter()
                    .map(|icon| icon.to_json())
                    .collect::<Vec<String>>()
                    .join(",\n")
            )
            .collect::<Vec<String>>()
            .join(",\n");

    format!(
        "{{
  \"images\" : [{}
  ],
  \"info\" : {{
    \"author\" : \"xcode\",
    \"version\" : 1
  }}
}}
", icons_json_str)
}

fn save_json(assets_path: &Path, json_str: &String) -> Result<()> {
    let path = assets_path.join("Contents.json");

    let mut file = File::create(&path).with_context(|| "Failed to create Contents.json")?;

    file.write_all(json_str.as_bytes()).with_context(|| "Failed to write to Contents.json")
}

fn get_tree<P: AsRef<Path>>(svg_path: P) -> Result<Tree> {
    let mut opt = usvg::Options::default();
    opt.resources_dir = std::fs::canonicalize(&svg_path).ok().and_then(|p| p.parent().map(|p| p.to_path_buf()));
    opt.fontdb.load_system_fonts();
    opt.fontdb.set_generic_families();

    let svg_data = std::fs::read(&svg_path)?;

    usvg::Tree::from_data(&svg_data, &opt)
        .with_context(|| "Failed to get svg tree")
}

fn generate_icon_files(assets_path: &Path, tree: &Tree, icons: &Vec<Icon>) -> Result<()> {
    for icon in icons {
        let size = icon.size * (icon.scale as f64);
        let size = size as u32;
        let path = assets_path.join(&icon.get_filename());
        let path: &str = path.to_str().with_context(|| "Failed to generate icon file path")?;
        save_png(&tree, size, path)?;
    }

    Ok(())
}

fn save_png<P: AsRef<Path>>(tree: &Tree, size: u32, path: P) -> Result<()> {
    let pixmap_size =
        usvg::Size::new(size as f64, size as f64)
            .with_context(|| "Failed to generate icon file")?
            .to_screen_size();

    let mut pixmap =
        tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
            .with_context(|| "Failed to genarate icon file")?;

    resvg::render(&tree, usvg::FitTo::Size(size, size), pixmap.as_mut())
        .with_context(|| "Failed to render svg image")?;

    pixmap.save_png(&path)?;

    Ok(())
}
