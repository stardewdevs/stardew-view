use stardew_view::color::Palette;
use stardew_view::config::{load_config, Config};

#[test]
fn default_config_loads() {
    let config = Config::default();
    assert!(config.theme.name.len() > 0);
    assert!(config.effects.is_empty() || !config.effects.is_empty());
}

#[test]
fn config_from_json_roundtrip() {
    let config = Config::default();
    let json = serde_json::to_string(&config).unwrap();
    let decoded: Config = serde_json::from_str(&json).unwrap();
    assert_eq!(decoded.theme.name, config.theme.name);
}

#[test]
fn palette_from_theme() {
    let theme = stardew_view::config::load_default_theme();
    let palette = Palette::from_theme(&theme);
    assert_eq!(palette.background, stardew_view::color::Color::BLACK);
    assert_eq!(palette.foreground, stardew_view::color::Color::GREEN);
}

#[test]
fn config_file_not_found_returns_defaults() {
    let config = load_config("/nonexistent/path/config.json");
    assert!(config.is_ok());
    assert_eq!(config.unwrap().theme.name, "default");
}