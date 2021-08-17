use once_cell::sync::Lazy;

pub static IOS_ICONS: Lazy<Vec<Icon>> = Lazy::new(|| vec![
    Icon::ios("40.png", "iphone", 2, 20.0),
    Icon::ios("60.png", "iphone", 3, 20.0),
    Icon::ios("58.png", "iphone", 2, 29.0),
    Icon::ios("87.png", "iphone", 3, 29.0),
    Icon::ios("80.png", "iphone", 2, 40.0),
    Icon::ios("120.png", "iphone", 3, 40.0),
    Icon::ios("120-1.png", "iphone", 2, 60.0),
    Icon::ios("180.png", "iphone", 3, 60.0),
    Icon::ios("20.png", "ipad", 1, 20.0),
    Icon::ios("40-1.png", "ipad", 2, 20.0),
    Icon::ios("29.png", "ipad", 1, 29.0),
    Icon::ios("58-1.png", "ipad", 2, 29.0),
    Icon::ios("40-2.png", "ipad", 1, 40.0),
    Icon::ios("80-1.png", "ipad", 2, 40.0),
    Icon::ios("76.png", "ipad", 1, 76.0),
    Icon::ios("152.png", "ipad", 2, 76.0),
    Icon::ios("167.png", "ipad", 2, 83.5),
    Icon::ios("1024.png", "ios-marketing", 1, 1024.0),
]);

pub static MAC_ICONS: Lazy<Vec<Icon>> = Lazy::new(|| vec![
    Icon::mac("16.png", "mac", 1, 16.0),
    Icon::mac("32.png", "mac", 2, 16.0),
    Icon::mac("32-1.png", "mac", 1, 32.0),
    Icon::mac("64.png", "mac", 2, 32.0),
    Icon::mac("128.png", "mac", 1, 128.0),
    Icon::mac("256.png", "mac", 2, 128.0),
    Icon::mac("256-1.png", "mac", 1, 256.0),
    Icon::mac("512.png", "mac", 2, 256.0),
    Icon::mac("512-1.png", "mac", 1, 512.0),
    Icon::mac("1024.png", "mac", 2, 512.0),
]);

pub static WATCH_ICONS: Lazy<Vec<Icon>> = Lazy::new(|| vec![
    Icon::watch("48.png", "watch", Some("notificationCenter"), 2, 24.0, Some(Subtype::_38)),
    Icon::watch("55.png", "watch", Some("notificationCenter"), 2, 27.5, Some(Subtype::_42)),
    Icon::watch("58.png", "watch", Some("companionSettings"), 2, 29.0, None),
    Icon::watch("87.png", "watch", Some("companionSettings"), 3, 29.0, None),
    Icon::watch("80.png", "watch", Some("appLauncher"), 2, 40.0, Some(Subtype::_38)),
    Icon::watch("88.png", "watch", Some("appLauncher"), 2, 44.0, Some(Subtype::_40)),
    Icon::watch("100.png", "watch", Some("appLauncher"), 2, 50.0, Some(Subtype::_44)),
    Icon::watch("172.png", "watch", Some("quickLook"), 2, 86.0, Some(Subtype::_38)),
    Icon::watch("196.png", "watch", Some("quickLook"), 2, 98.0, Some(Subtype::_42)),
    Icon::watch("216.png", "watch", Some("quickLook"), 2, 108.0, Some(Subtype::_44)),
    Icon::watch("1024.png", "watch-marketing", None, 1, 1024.0, None),
]);

pub enum Subtype {
    _38 = 38,
    _40 = 40,
    _42 = 42,
    _44 = 44,
}

impl Subtype {
    pub fn to_string(&self) -> &str {
        match self {
            Subtype::_38 => "38mm",
            Subtype::_40 => "40mm",
            Subtype::_42 => "42mm",
            Subtype::_44 => "44mm",
        }
    }
}

pub struct Icon {
    pub filename: String,
    pub idiom: String,
    pub role: Option<String>,
    pub scale: u32,
    pub size: f64,
    pub subtype: Option<Subtype>,
}

impl Icon {
    pub fn ios(filename: &str, idiom: &str, scale: u32, size: f64) -> Self {
        Icon { filename: filename.to_string(), idiom: idiom.to_string(), role: None, scale, size, subtype: None }
    }

    pub fn mac(filename: &str, idiom: &str, scale: u32, size: f64) -> Self {
        Self::ios(filename, idiom, scale, size)
    }

    pub fn watch(filename: &str, idiom: &str, role: Option<&str>, scale: u32, size: f64, subtype: Option<Subtype>) -> Self {
        Icon { filename: filename.to_string(), idiom: idiom.to_string(), role: role.map(|v| v.to_string()), scale, size, subtype }
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
            , self.filename, self.idiom, role, self.scale, self.size, self.size, subtype
        )
    }
}
