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
                r: 136,
                g: 192,
                b: 208,
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
                r: 163,
                g: 190,
                b: 140,
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
                r: 129,
                g: 161,
                b: 193,
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
                r: 235,
                g: 203,
                b: 139,
            }), // Nord yellow background
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
                r: 163,
                g: 190,
                b: 140,
            }), // Nord green background
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
                r: 136,
                g: 192,
                b: 208,
            }), // Nord cyan background
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}
