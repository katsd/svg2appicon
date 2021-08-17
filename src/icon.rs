use once_cell::sync::Lazy;

pub static IOS_ICONS: Lazy<Vec<Icon>> = Lazy::new(|| vec![
    Icon::ios("iphone", 2, 20.0),
    Icon::ios("iphone", 3, 20.0),
    Icon::ios("iphone", 2, 29.0),
    Icon::ios("iphone", 3, 29.0),
    Icon::ios("iphone", 2, 40.0),
    Icon::ios("iphone", 3, 40.0),
    Icon::ios("iphone", 2, 60.0),
    Icon::ios("iphone", 3, 60.0),
    Icon::ios("ipad", 1, 20.0),
    Icon::ios("ipad", 2, 20.0),
    Icon::ios("ipad", 1, 29.0),
    Icon::ios("ipad", 2, 29.0),
    Icon::ios("ipad", 1, 40.0),
    Icon::ios("ipad", 2, 40.0),
    Icon::ios("ipad", 1, 76.0),
    Icon::ios("ipad", 2, 76.0),
    Icon::ios("ipad", 2, 83.5),
    Icon::ios("ios-marketing", 1, 1024.0),
]);

pub static MAC_ICONS: Lazy<Vec<Icon>> = Lazy::new(|| vec![
    Icon::mac("mac", 1, 16.0),
    Icon::mac("mac", 2, 16.0),
    Icon::mac("mac", 1, 32.0),
    Icon::mac("mac", 2, 32.0),
    Icon::mac("mac", 1, 128.0),
    Icon::mac("mac", 2, 128.0),
    Icon::mac("mac", 1, 256.0),
    Icon::mac("mac", 2, 256.0),
    Icon::mac("mac", 1, 512.0),
    Icon::mac("mac", 2, 512.0),
]);

pub static WATCH_ICONS: Lazy<Vec<Icon>> = Lazy::new(|| vec![
    Icon::watch("watch", Some("notificationCenter"), 2, 24.0, Some(Subtype::_38)),
    Icon::watch("watch", Some("notificationCenter"), 2, 27.5, Some(Subtype::_42)),
    Icon::watch("watch", Some("companionSettings"), 2, 29.0, None),
    Icon::watch("watch", Some("companionSettings"), 3, 29.0, None),
    Icon::watch("watch", Some("appLauncher"), 2, 40.0, Some(Subtype::_38)),
    Icon::watch("watch", Some("appLauncher"), 2, 44.0, Some(Subtype::_40)),
    Icon::watch("watch", Some("appLauncher"), 2, 50.0, Some(Subtype::_44)),
    Icon::watch("watch", Some("quickLook"), 2, 86.0, Some(Subtype::_38)),
    Icon::watch("watch", Some("quickLook"), 2, 98.0, Some(Subtype::_42)),
    Icon::watch("watch", Some("quickLook"), 2, 108.0, Some(Subtype::_44)),
    Icon::watch("watch-marketing", None, 1, 1024.0, None),
]);

enum OS {
    Ios,
    Macos,
    Watchos,
}

impl OS {
    fn get_str(&self) -> String {
        match self {
            OS::Ios => "iOS".to_string(),
            OS::Macos => "macOS".to_string(),
            OS::Watchos => "watchOS".to_string(),
        }
    }
}

enum Subtype {
    _38 = 38,
    _40 = 40,
    _42 = 42,
    _44 = 44,
}

impl Subtype {
    fn to_string(&self) -> &str {
        match self {
            Subtype::_38 => "38mm",
            Subtype::_40 => "40mm",
            Subtype::_42 => "42mm",
            Subtype::_44 => "44mm",
        }
    }
}

pub struct Icon {
    os: OS,
    idiom: String,
    role: Option<String>,
    pub scale: u32,
    pub size: f64,
    subtype: Option<Subtype>,
}

impl Icon {
    fn ios(idiom: &str, scale: u32, size: f64) -> Self {
        Icon { os: OS::Ios, idiom: idiom.to_string(), role: None, scale, size, subtype: None }
    }

    fn mac(idiom: &str, scale: u32, size: f64) -> Self {
        Icon { os: OS::Macos, idiom: idiom.to_string(), role: None, scale, size, subtype: None }
    }

    fn watch(idiom: &str, role: Option<&str>, scale: u32, size: f64, subtype: Option<Subtype>) -> Self {
        Icon { os: OS::Watchos, idiom: idiom.to_string(), role: role.map(|v| v.to_string()), scale, size, subtype }
    }

    pub fn to_json(&self) -> String {
        let role = self.role.as_ref().map_or("".to_string(), |role| { format!("\n        \"role\" : \"{}\",", role) });
        let subtype = self.subtype.as_ref().map_or("".to_string(), |subtype| { format!(",\n        \"subtype\" : \"{}\"", subtype.to_string()) });

        format!(
            "    {{
        \"filename\" : \"{}\",
        \"idiom\" : \"{}\",{}
        \"scale\" : \"{}x\",
        \"size\" : \"{}x{}\"{}
    }}"
            , self.get_filename(), self.idiom, role, self.scale, self.size, self.size, subtype
        )
    }

    pub fn get_filename(&self) -> String {
        format!("{}-{}x{}@{}x.png", self.os.get_str(), self.size, self.size, self.scale)
    }
}
