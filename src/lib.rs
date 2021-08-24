pub mod icon;

use usvg::Tree;
use std::path::{Path, PathBuf};
use std::fs::{self, File};
use std::io::Write;
use anyhow::{Context, Result};
use icon::Icon;

pub enum SVG {
    Data(Vec<u8>),
    File(PathBuf),
}

pub struct Config {
    pub svg: SVG,
    pub assets_path: String,
    pub ios: bool,
    pub mac: bool,
    pub watch: bool,
}

pub fn generate_icons(config: &Config) {
    let tree = match &config.svg {
        SVG::Data(data) =>
            match get_tree_from_data(data) {
                Ok(tree) => tree,
                Err(e) => panic!("{}", e)
            }
        SVG::File(p) =>
            match get_tree(p) {
                Ok(tree) => tree,
                Err(e) => panic!("{}", e)
            }
    };

    if Path::new(&config.assets_path).exists() {
        fs::remove_dir_all(&config.assets_path)
            .expect(&format!("Failed to remove {}", config.assets_path));
    }
    fs::create_dir(&config.assets_path)
        .expect(&format!("Failed to create {}", config.assets_path));

    let icons_set = get_icons_set(&config);

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


fn get_icons_set(config: &Config) -> Vec<&Vec<Icon>> {
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

    icons_set
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
  \"images\" : [
{}
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
    let mut opt = get_options();
    opt.resources_dir = std::fs::canonicalize(&svg_path).ok().and_then(|p| p.parent().map(|p| p.to_path_buf()));

    let svg_data = std::fs::read(&svg_path)?;

    usvg::Tree::from_data(&svg_data, &opt)
        .with_context(|| "Failed to get svg tree")
}

fn get_tree_from_data(svg_data: &Vec<u8>) -> Result<Tree> {
    let opt = get_options();
    usvg::Tree::from_data(svg_data, &opt)
        .with_context(|| "Failed to get svg tree")
}

fn get_options() -> usvg::Options {
    let mut opt = usvg::Options::default();
    opt.fontdb.load_system_fonts();
    opt.font_family = "Helvetica".to_string();
    opt
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
