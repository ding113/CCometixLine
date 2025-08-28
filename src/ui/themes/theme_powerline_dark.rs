use crate::config::{
    AnsiColor, ColorConfig, IconConfig, SegmentConfig, SegmentId, TextStyleConfig,
};
use std::collections::HashMap;

pub fn model_segment() -> SegmentConfig {
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

pub fn directory_segment() -> SegmentConfig {
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

pub fn git_segment() -> SegmentConfig {
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

pub fn usage_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Usage,
        enabled: true,
        icon: IconConfig {
            plain: "⚡️".to_string(),
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

pub fn cost_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Cost,
        enabled: false,
        icon: IconConfig {
            plain: "💰".to_string(),
            nerd_font: "\u{eec1}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 229,
                g: 192,
                b: 123,
            }),
            text: Some(AnsiColor::Rgb {
                r: 229,
                g: 192,
                b: 123,
            }),
            background: Some(AnsiColor::Rgb {
                r: 40,
                g: 44,
                b: 52,
            }), // Powerline dark background
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn session_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::Session,
        enabled: false,
        icon: IconConfig {
            plain: "⏱️".to_string(),
            nerd_font: "\u{f19bb}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 163,
                g: 190,
                b: 140,
            }),
            text: Some(AnsiColor::Rgb {
                r: 163,
                g: 190,
                b: 140,
            }),
            background: Some(AnsiColor::Rgb {
                r: 45,
                g: 50,
                b: 59,
            }), // Powerline darker background
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}

pub fn output_style_segment() -> SegmentConfig {
    SegmentConfig {
        id: SegmentId::OutputStyle,
        enabled: false,
        icon: IconConfig {
            plain: "🎯".to_string(),
            nerd_font: "\u{f12f5}".to_string(),
        },
        colors: ColorConfig {
            icon: Some(AnsiColor::Rgb {
                r: 129,
                g: 161,
                b: 193,
            }),
            text: Some(AnsiColor::Rgb {
                r: 129,
                g: 161,
                b: 193,
            }),
            background: Some(AnsiColor::Rgb {
                r: 50,
                g: 56,
                b: 66,
            }), // Powerline darkest background
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}
