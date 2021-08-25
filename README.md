# svg2appicon

Convert svg image to icons for iOS, macOS, and watchOS

## Usage
```
svg2appicon [OPTIONS] <APPICONSET>

OPTIONS:
        --ios <SVG_FILE>      Generate icons for iOS
        --mac <SVG_FILE>      Generate icons for macOS
        --watch <SVG_FILE>    Generate icons for watchOS

ARGS:
    <APPICONSET>    Path to .appiconset (e.g. /path/to/Assets.xcassets/AppIcon.appiconset)
```

### Example
```
svg2appicon ~/Project/Project/Assets.xcassets/AppIcon.appiconset --ios ~/ios_icon.svg --mac ~/mac_icon.svg 
```

## Install
```
cargo install svg2appicon
```
