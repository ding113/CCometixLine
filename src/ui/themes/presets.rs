// Theme presets for TUI configuration

use crate::config::{Config, StyleConfig, StyleMode, SegmentConfig, SegmentId, IconConfig, ColorConfig, AnsiColor, TextStyleConfig};
use std::collections::HashMap;

// Import all theme modules
use super::{
    theme_cometix, theme_default, theme_gruvbox, theme_minimal, theme_nord, theme_powerline_dark,
    theme_powerline_light, theme_powerline_rose_pine, theme_powerline_tokyo_night,
};

pub struct ThemePresets;

impl ThemePresets {
    pub fn get_theme(theme_name: &str) -> Config {
        // First try to load from file
        if let Ok(config) = Self::load_theme_from_file(theme_name) {
            return config;
        }

        // Fallback to built-in themes
        match theme_name {
            "cometix" => Self::get_cometix(),
            "default" => Self::get_default(),
            "gruvbox" => Self::get_gruvbox(),
            "minimal" => Self::get_minimal(),
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
            "cometix".to_string(),
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
            ("cometix", "Cometix theme"),
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

    pub fn get_cometix() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: " | ".to_string(),
            },
            segments: vec![
                theme_cometix::model_segment(),
                theme_cometix::directory_segment(),
                theme_cometix::git_segment(),
                theme_cometix::usage_segment(),
                theme_cometix::cost_segment(),
                theme_cometix::session_segment(),
                theme_cometix::output_style_segment(),
                // PackyCode quota segment
                Self::quota_segment(),
            ],
            theme: "cometix".to_string(),
        }
    }

    pub fn get_default() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::Plain,
                separator: " | ".to_string(),
            },
            segments: vec![
                theme_default::model_segment(),
                theme_default::directory_segment(),
                theme_default::git_segment(),
                theme_default::usage_segment(),
                theme_default::cost_segment(),
                theme_default::session_segment(),
                theme_default::output_style_segment(),
                // PackyCode quota segment
                Self::quota_segment(),
            ],
            theme: "default".to_string(),
        }
    }

    pub fn get_minimal() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::Plain,
                separator: " │ ".to_string(),
            },
            segments: vec![
                theme_minimal::model_segment(),
                theme_minimal::directory_segment(),
                theme_minimal::git_segment(),
                theme_minimal::usage_segment(),
                theme_minimal::cost_segment(),
                theme_minimal::session_segment(),
                theme_minimal::output_style_segment(),
                // PackyCode quota segment
                Self::quota_segment(),
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
                theme_gruvbox::model_segment(),
                theme_gruvbox::directory_segment(),
                theme_gruvbox::git_segment(),
                theme_gruvbox::usage_segment(),
                theme_gruvbox::cost_segment(),
                theme_gruvbox::session_segment(),
                theme_gruvbox::output_style_segment(),
                // PackyCode quota segment
                Self::quota_segment(),
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
                theme_nord::model_segment(),
                theme_nord::directory_segment(),
                theme_nord::git_segment(),
                theme_nord::usage_segment(),
                theme_nord::cost_segment(),
                theme_nord::session_segment(),
                theme_nord::output_style_segment(),
                // PackyCode quota segment
                Self::quota_segment(),
            ],
            theme: "nord".to_string(),
        }
    }

    pub fn get_powerline_dark() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                theme_powerline_dark::model_segment(),
                theme_powerline_dark::directory_segment(),
                theme_powerline_dark::git_segment(),
                theme_powerline_dark::usage_segment(),
                theme_powerline_dark::cost_segment(),
                theme_powerline_dark::session_segment(),
                theme_powerline_dark::output_style_segment(),
                // PackyCode quota segment
                Self::quota_segment(),
            ],
            theme: "powerline-dark".to_string(),
        }
    }

    pub fn get_powerline_light() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                theme_powerline_light::model_segment(),
                theme_powerline_light::directory_segment(),
                theme_powerline_light::git_segment(),
                theme_powerline_light::usage_segment(),
                theme_powerline_light::cost_segment(),
                theme_powerline_light::session_segment(),
                theme_powerline_light::output_style_segment(),
                // PackyCode quota segment
                Self::quota_segment(),
            ],
            theme: "powerline-light".to_string(),
        }
    }

    pub fn get_powerline_rose_pine() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                theme_powerline_rose_pine::model_segment(),
                theme_powerline_rose_pine::directory_segment(),
                theme_powerline_rose_pine::git_segment(),
                theme_powerline_rose_pine::usage_segment(),
                theme_powerline_rose_pine::cost_segment(),
                theme_powerline_rose_pine::session_segment(),
                theme_powerline_rose_pine::output_style_segment(),
                // PackyCode quota segment
                Self::quota_segment(),
            ],
            theme: "powerline-rose-pine".to_string(),
        }
    }

    pub fn get_powerline_tokyo_night() -> Config {
        Config {
            style: StyleConfig {
                mode: StyleMode::NerdFont,
                separator: "".to_string(),
            },
            segments: vec![
                theme_powerline_tokyo_night::model_segment(),
                theme_powerline_tokyo_night::directory_segment(),
                theme_powerline_tokyo_night::git_segment(),
                theme_powerline_tokyo_night::usage_segment(),
                theme_powerline_tokyo_night::cost_segment(),
                theme_powerline_tokyo_night::session_segment(),
                theme_powerline_tokyo_night::output_style_segment(),
                // PackyCode quota segment
                Self::quota_segment(),
            ],
            theme: "powerline-tokyo-night".to_string(),
        }
    }

    // PackyCode-specific quota segment for all themes
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
