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
                r: 246,
                g: 193,
                b: 119,
            }),
            text: Some(AnsiColor::Rgb {
                r: 246,
                g: 193,
                b: 119,
            }),
            background: Some(AnsiColor::Rgb {
                r: 35,
                g: 33,
                b: 54,
            }), // Rose Pine dark background
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
                r: 42,
                g: 39,
                b: 63,
            }), // Rose Pine darker background
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
                r: 49,
                g: 116,
                b: 143,
            }),
            text: Some(AnsiColor::Rgb {
                r: 49,
                g: 116,
                b: 143,
            }),
            background: Some(AnsiColor::Rgb {
                r: 38,
                g: 35,
                b: 58,
            }), // Rose Pine darkest background
        },
        styles: TextStyleConfig::default(),
        options: HashMap::new(),
    }
}
