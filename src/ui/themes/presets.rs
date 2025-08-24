// Theme presets for TUI configuration

use crate::config::{
    AnsiColor, ColorConfig, Config, IconConfig, SegmentConfig, SegmentId, StyleConfig, StyleMode,
    TextStyleConfig,
};
use std::collections::HashMap;

pub struct ThemePresets;

impl ThemePresets {
    pub fn get_theme(theme_name: &str) -> Config {
        // First try to load from file
        if let Ok(config) = Self::load_theme_from_file(theme_name) {
            return config;
        }

        // Fallback to built-in themes
        match theme_name {
            "minimal" => Self::get_minimal(),
            "gruvbox" => Self::get_gruvbox(),
            "nord" => Self::get_nord(),
            "powerline-dark" => Self::get_powerline_dark(),
            "powerline-light" => Self::get_powerline_light(),
            "powerline-rose-pine" => Self::get_powerline_rose_pine(),
            "powerline-tokyo-night" => Self::get_powerline_tokyo_night(),
            _ => Self::get_default(),
        }
    }

    /// Load theme from file system
    pub fn load_theme_from_file(theme_name: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let themes_dir = Self::get_themes_path();
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));

        if !theme_path.exists() {
            return Err(format!("Theme file not found: {}", theme_path.display()).into());
        }

        let content = std::fs::read_to_string(&theme_path)?;
        let mut config: Config = toml::from_str(&content)?;

        // Ensure the theme field matches the requested theme
        config.theme = theme_name.to_string();

        Ok(config)
    }

    /// Get the themes directory path (~/.claude/ccline/themes/)
    fn get_themes_path() -> std::path::PathBuf {
        if let Some(home) = dirs::home_dir() {
            home.join(".claude").join("ccline").join("themes")
        } else {
            std::path::PathBuf::from(".claude/ccline/themes")
        }
    }

    /// Save current config as a new theme
    pub fn save_theme(theme_name: &str, config: &Config) -> Result<(), Box<dyn std::error::Error>> {
        let themes_dir = Self::get_themes_path();
        let theme_path = themes_dir.join(format!("{}.toml", theme_name));

        // Create themes directory if it doesn't exist
        std::fs::create_dir_all(&themes_dir)?;

        // Create a copy of config with the correct theme name
        let mut theme_config = config.clone();
        theme_config.theme = theme_name.to_string();

        let content = toml::to_string_pretty(&theme_config)?;
        std::fs::write(&theme_path, content)?;

        Ok(())
    }

    /// List all available themes (built-in + custom)
    pub fn list_available_themes() -> Vec<String> {
        let mut themes = vec![
            "default".to_string(),
            "minimal".to_string(),
            "gruvbox".to_string(),
            "nord".to_string(),
            "powerline-dark".to_string(),
            "powerline-light".to_string(),
            "powerline-rose-pine".to_string(),
            "powerline-tokyo-night".to_string(),
        ];

        // Add custom themes from file system
        if let Ok(themes_dir) = std::fs::read_dir(Self::get_themes_path()) {
            for entry in themes_dir.flatten() {
                if let Some(name) = entry.file_name().to_str() {
                    if name.ends_with(".toml") {
                        let theme_name = name.trim_end_matches(".toml").to_string();
                        if !themes.contains(&theme_name) {
                            themes.push(theme_name);
                        }
                    }
                }
            }
        }

        themes
    }

    pub fn get_available_themes() -> Vec<(&'static str, &'static str)> {
        vec![
            ("default", "Default theme with emoji icons"),
            ("minimal", "Minimal theme with reduced colors"),
            ("gruvbox", "Gruvbox color scheme"),
            ("nord", "Nord color scheme"),
            ("powerline-dark", "Dark powerline theme"),
            ("powerline-light", "Light powerline theme"),
            ("powerline-rose-pine", "Rose Pine powerline theme"),
            ("powerline-tokyo-night", "Tokyo Night powerline theme"),
        ]
    }

    pub fn get_default() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::Plain,
                separator: " | ".to_string(),
            },
            segments: vec![
                Self::model_segment(),
                Self::directory_segment(),
                Self::git_segment(),
                Self::usage_segment(),
                Self::quota_segment(),
            ],
            theme: "default".to_string(),
        }
    }

    fn model_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Model,
            enabled: true,
            icon: IconConfig {
                plain: "🤖".to_string(),
                nerd_font: "\u{e26d}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 14 }), // Cyan
                text: Some(AnsiColor::Color16 { c16: 14 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn directory_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Directory,
            enabled: true,
            icon: IconConfig {
                plain: "📁".to_string(),
                nerd_font: "\u{f024b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 11 }), // Yellow
                text: Some(AnsiColor::Color16 { c16: 10 }), // Green
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn git_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Git,
            enabled: true,
            icon: IconConfig {
                plain: "🌿".to_string(),
                nerd_font: "\u{f02a2}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 12 }), // Blue
                text: Some(AnsiColor::Color16 { c16: 12 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: {
                let mut opts = HashMap::new();
                opts.insert("show_sha".to_string(), serde_json::Value::Bool(false));
                opts
            },
        }
    }

    fn usage_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Usage,
            enabled: true,
            icon: IconConfig {
                plain: "⚡".to_string(),
                nerd_font: "\u{f49b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 13 }), // Magenta
                text: Some(AnsiColor::Color16 { c16: 13 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    pub fn get_minimal() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::Plain,
                separator: " │ ".to_string(), // Thin vertical bar
            },
            segments: vec![
                Self::minimal_model_segment(),
                Self::minimal_directory_segment(),
                Self::minimal_git_segment(),
                Self::minimal_usage_segment(),
                Self::minimal_quota_segment(),
            ],
            theme: "minimal".to_string(),
        }
    }

    pub fn get_gruvbox() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: " | ".to_string(),
            },
            segments: vec![
                Self::gruvbox_model_segment(),
                Self::gruvbox_directory_segment(),
                Self::gruvbox_git_segment(),
                Self::gruvbox_usage_segment(),
                Self::gruvbox_quota_segment(),
            ],
            theme: "gruvbox".to_string(),
        }
    }

    pub fn get_nord() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                Self::nord_model_segment(),
                Self::nord_directory_segment(),
                Self::nord_git_segment(),
                Self::nord_usage_segment(),
                Self::nord_quota_segment(),
            ],
            theme: "nord".to_string(),
        }
    }

    // Minimal theme segments
    fn minimal_model_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Model,
            enabled: true,
            icon: IconConfig {
                plain: "✽".to_string(),
                nerd_font: "\u{f2d0}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 7 }),
                text: Some(AnsiColor::Color16 { c16: 7 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn minimal_directory_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Directory,
            enabled: true,
            icon: IconConfig {
                plain: "~".to_string(),
                nerd_font: "\u{f024b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 8 }),
                text: Some(AnsiColor::Color16 { c16: 7 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn minimal_git_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Git,
            enabled: true,
            icon: IconConfig {
                plain: "⑂".to_string(),
                nerd_font: "\u{f02a2}".to_string(),
            },
            colors: ColorConfig {
                icon: None,
                text: Some(AnsiColor::Color16 { c16: 8 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: {
                let mut opts = HashMap::new();
                opts.insert("show_sha".to_string(), serde_json::Value::Bool(false));
                opts
            },
        }
    }

    fn minimal_usage_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Usage,
            enabled: true,
            icon: IconConfig {
                plain: "◐".to_string(),
                nerd_font: "\u{f49b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 13 }),
                text: Some(AnsiColor::Color16 { c16: 13 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn minimal_quota_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Quota,
            enabled: true,
            icon: IconConfig {
                plain: "$".to_string(),
                nerd_font: "\u{f155}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 8 }),
                text: Some(AnsiColor::Color16 { c16: 8 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    // Gruvbox theme segments
    fn gruvbox_model_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Model,
            enabled: true,
            icon: IconConfig {
                plain: "🤖".to_string(),
                nerd_font: "\u{e26d}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 14 }),
                text: Some(AnsiColor::Color16 { c16: 14 }),
                background: None,
            },
            styles: TextStyleConfig { text_bold: true },
            options: HashMap::new(),
        }
    }

    fn gruvbox_directory_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Directory,
            enabled: true,
            icon: IconConfig {
                plain: "📁".to_string(),
                nerd_font: "\u{f024b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 11 }),
                text: Some(AnsiColor::Color16 { c16: 10 }),
                background: None,
            },
            styles: TextStyleConfig { text_bold: true },
            options: HashMap::new(),
        }
    }

    fn gruvbox_git_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Git,
            enabled: true,
            icon: IconConfig {
                plain: "🌿".to_string(),
                nerd_font: "\u{f02a2}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 4 }),
                text: Some(AnsiColor::Color16 { c16: 4 }),
                background: None,
            },
            styles: TextStyleConfig { text_bold: true },
            options: {
                let mut opts = HashMap::new();
                opts.insert("show_sha".to_string(), serde_json::Value::Bool(false));
                opts
            },
        }
    }

    fn gruvbox_usage_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Usage,
            enabled: true,
            icon: IconConfig {
                plain: "⚡".to_string(),
                nerd_font: "\u{f49b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 5 }),
                text: Some(AnsiColor::Color16 { c16: 5 }),
                background: None,
            },
            styles: TextStyleConfig { text_bold: true },
            options: HashMap::new(),
        }
    }

    fn gruvbox_quota_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Quota,
            enabled: true,
            icon: IconConfig {
                plain: "💰".to_string(),
                nerd_font: "\u{f155}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 3 }),
                text: Some(AnsiColor::Color16 { c16: 3 }),
                background: None,
            },
            styles: TextStyleConfig { text_bold: true },
            options: HashMap::new(),
        }
    }

    // Nord theme segments
    fn nord_model_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Model,
            enabled: true,
            icon: IconConfig {
                plain: "🤖".to_string(),
                nerd_font: "\u{e26d}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 191,
                    g: 97,
                    b: 106,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 191,
                    g: 97,
                    b: 106,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 76,
                    g: 86,
                    b: 106,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn nord_directory_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Directory,
            enabled: true,
            icon: IconConfig {
                plain: "📁".to_string(),
                nerd_font: "\u{f024b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 235,
                    g: 203,
                    b: 139,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 163,
                    g: 190,
                    b: 140,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 67,
                    g: 76,
                    b: 94,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn nord_git_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Git,
            enabled: true,
            icon: IconConfig {
                plain: "🌿".to_string(),
                nerd_font: "\u{f02a2}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 136,
                    g: 192,
                    b: 208,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 136,
                    g: 192,
                    b: 208,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 59,
                    g: 66,
                    b: 82,
                }),
            },
            styles: TextStyleConfig::default(),
            options: {
                let mut opts = HashMap::new();
                opts.insert("show_sha".to_string(), serde_json::Value::Bool(false));
                opts
            },
        }
    }

    fn nord_usage_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Usage,
            enabled: true,
            icon: IconConfig {
                plain: "⚡".to_string(),
                nerd_font: "\u{f49b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 46,
                    g: 52,
                    b: 64,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 46,
                    g: 52,
                    b: 64,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 180,
                    g: 142,
                    b: 173,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn nord_quota_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Quota,
            enabled: true,
            icon: IconConfig {
                plain: "💰".to_string(),
                nerd_font: "\u{f155}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 208,
                    g: 135,
                    b: 112,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 208,
                    g: 135,
                    b: 112,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 76,
                    g: 86,
                    b: 106,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    // Powerline Dark theme
    pub fn get_powerline_dark() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                Self::powerline_dark_model_segment(),
                Self::powerline_dark_directory_segment(),
                Self::powerline_dark_git_segment(),
                Self::powerline_dark_usage_segment(),
                Self::powerline_dark_quota_segment(),
            ],
            theme: "powerline-dark".to_string(),
        }
    }

    fn powerline_dark_model_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Model,
            enabled: true,
            icon: IconConfig {
                plain: "🤖".to_string(),
                nerd_font: "\u{e26d}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 45,
                    g: 45,
                    b: 45,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn powerline_dark_directory_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Directory,
            enabled: true,
            icon: IconConfig {
                plain: "📁".to_string(),
                nerd_font: "\u{f024b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 139,
                    g: 69,
                    b: 19,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn powerline_dark_git_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Git,
            enabled: true,
            icon: IconConfig {
                plain: "🌿".to_string(),
                nerd_font: "\u{f02a2}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 64,
                    g: 64,
                    b: 64,
                }),
            },
            styles: TextStyleConfig::default(),
            options: {
                let mut opts = HashMap::new();
                opts.insert("show_sha".to_string(), serde_json::Value::Bool(false));
                opts
            },
        }
    }

    fn powerline_dark_usage_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Usage,
            enabled: true,
            icon: IconConfig {
                plain: "⚡".to_string(),
                nerd_font: "\u{f49b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 209,
                    g: 213,
                    b: 219,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 209,
                    g: 213,
                    b: 219,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 55,
                    g: 65,
                    b: 81,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn powerline_dark_quota_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Quota,
            enabled: true,
            icon: IconConfig {
                plain: "💰".to_string(),
                nerd_font: "\u{f155}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 85,
                    g: 85,
                    b: 85,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    // Powerline Light theme
    pub fn get_powerline_light() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                Self::powerline_light_model_segment(),
                Self::powerline_light_directory_segment(),
                Self::powerline_light_git_segment(),
                Self::powerline_light_usage_segment(),
                Self::powerline_light_quota_segment(),
            ],
            theme: "powerline-light".to_string(),
        }
    }

    fn powerline_light_model_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Model,
            enabled: true,
            icon: IconConfig {
                plain: "🤖".to_string(),
                nerd_font: "\u{e26d}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb { r: 0, g: 0, b: 0 }),
                text: Some(AnsiColor::Rgb { r: 0, g: 0, b: 0 }),
                background: Some(AnsiColor::Rgb {
                    r: 135,
                    g: 206,
                    b: 235,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn powerline_light_directory_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Directory,
            enabled: true,
            icon: IconConfig {
                plain: "📁".to_string(),
                nerd_font: "\u{f024b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 107,
                    b: 71,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn powerline_light_git_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Git,
            enabled: true,
            icon: IconConfig {
                plain: "🌿".to_string(),
                nerd_font: "\u{f02a2}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 79,
                    g: 179,
                    b: 217,
                }),
            },
            styles: TextStyleConfig::default(),
            options: {
                let mut opts = HashMap::new();
                opts.insert("show_sha".to_string(), serde_json::Value::Bool(false));
                opts
            },
        }
    }

    fn powerline_light_usage_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Usage,
            enabled: true,
            icon: IconConfig {
                plain: "⚡".to_string(),
                nerd_font: "\u{f49b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 107,
                    g: 114,
                    b: 128,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn powerline_light_quota_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Quota,
            enabled: true,
            icon: IconConfig {
                plain: "💰".to_string(),
                nerd_font: "\u{f155}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 255,
                    g: 255,
                    b: 255,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 76,
                    g: 175,
                    b: 80,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    // Powerline Rose Pine theme
    pub fn get_powerline_rose_pine() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                Self::powerline_rose_pine_model_segment(),
                Self::powerline_rose_pine_directory_segment(),
                Self::powerline_rose_pine_git_segment(),
                Self::powerline_rose_pine_usage_segment(),
                Self::powerline_rose_pine_quota_segment(),
            ],
            theme: "powerline-rose-pine".to_string(),
        }
    }

    fn powerline_rose_pine_model_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Model,
            enabled: true,
            icon: IconConfig {
                plain: "🤖".to_string(),
                nerd_font: "\u{e26d}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 235,
                    g: 188,
                    b: 186,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 235,
                    g: 188,
                    b: 186,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 25,
                    g: 23,
                    b: 36,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn powerline_rose_pine_directory_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Directory,
            enabled: true,
            icon: IconConfig {
                plain: "📁".to_string(),
                nerd_font: "\u{f024b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 196,
                    g: 167,
                    b: 231,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 196,
                    g: 167,
                    b: 231,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 38,
                    g: 35,
                    b: 58,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn powerline_rose_pine_git_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Git,
            enabled: true,
            icon: IconConfig {
                plain: "🌿".to_string(),
                nerd_font: "\u{f02a2}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 156,
                    g: 207,
                    b: 216,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 156,
                    g: 207,
                    b: 216,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 31,
                    g: 29,
                    b: 46,
                }),
            },
            styles: TextStyleConfig::default(),
            options: {
                let mut opts = HashMap::new();
                opts.insert("show_sha".to_string(), serde_json::Value::Bool(false));
                opts
            },
        }
    }

    fn powerline_rose_pine_usage_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Usage,
            enabled: true,
            icon: IconConfig {
                plain: "⚡".to_string(),
                nerd_font: "\u{f49b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 224,
                    g: 222,
                    b: 244,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 224,
                    g: 222,
                    b: 244,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 82,
                    g: 79,
                    b: 103,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn powerline_rose_pine_quota_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Quota,
            enabled: true,
            icon: IconConfig {
                plain: "💰".to_string(),
                nerd_font: "\u{f155}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 235,
                    g: 188,
                    b: 186,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 235,
                    g: 188,
                    b: 186,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 49,
                    g: 46,
                    b: 65,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    // Powerline Tokyo Night theme
    pub fn get_powerline_tokyo_night() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                Self::powerline_tokyo_night_model_segment(),
                Self::powerline_tokyo_night_directory_segment(),
                Self::powerline_tokyo_night_git_segment(),
                Self::powerline_tokyo_night_usage_segment(),
                Self::powerline_tokyo_night_quota_segment(),
            ],
            theme: "powerline-tokyo-night".to_string(),
        }
    }

    fn powerline_tokyo_night_model_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Model,
            enabled: true,
            icon: IconConfig {
                plain: "🤖".to_string(),
                nerd_font: "\u{e26d}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 252,
                    g: 167,
                    b: 234,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 252,
                    g: 167,
                    b: 234,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 25,
                    g: 27,
                    b: 41,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn powerline_tokyo_night_directory_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Directory,
            enabled: true,
            icon: IconConfig {
                plain: "📁".to_string(),
                nerd_font: "\u{f024b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 130,
                    g: 170,
                    b: 255,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 130,
                    g: 170,
                    b: 255,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 47,
                    g: 51,
                    b: 77,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn powerline_tokyo_night_git_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Git,
            enabled: true,
            icon: IconConfig {
                plain: "🌿".to_string(),
                nerd_font: "\u{f02a2}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 195,
                    g: 232,
                    b: 141,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 195,
                    g: 232,
                    b: 141,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 30,
                    g: 32,
                    b: 48,
                }),
            },
            styles: TextStyleConfig::default(),
            options: {
                let mut opts = HashMap::new();
                opts.insert("show_sha".to_string(), serde_json::Value::Bool(false));
                opts
            },
        }
    }

    fn powerline_tokyo_night_usage_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Usage,
            enabled: true,
            icon: IconConfig {
                plain: "⚡".to_string(),
                nerd_font: "\u{f49b}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 192,
                    g: 202,
                    b: 245,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 192,
                    g: 202,
                    b: 245,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 61,
                    g: 89,
                    b: 161,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn powerline_tokyo_night_quota_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Quota,
            enabled: true,
            icon: IconConfig {
                plain: "💰".to_string(),
                nerd_font: "\u{f155}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Rgb {
                    r: 247,
                    g: 118,
                    b: 142,
                }),
                text: Some(AnsiColor::Rgb {
                    r: 247,
                    g: 118,
                    b: 142,
                }),
                background: Some(AnsiColor::Rgb {
                    r: 36,
                    g: 40,
                    b: 59,
                }),
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }

    fn quota_segment() -> SegmentConfig {
        SegmentConfig {
            id: SegmentId::Quota,
            enabled: true,
            icon: IconConfig {
                plain: "💰".to_string(),
                nerd_font: "\u{f155}".to_string(),
            },
            colors: ColorConfig {
                icon: Some(AnsiColor::Color16 { c16: 11 }), // Yellow
                text: Some(AnsiColor::Color16 { c16: 11 }),
                background: None,
            },
            styles: TextStyleConfig::default(),
            options: HashMap::new(),
        }
    }
}
