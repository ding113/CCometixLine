use super::{Segment, SegmentData};
use crate::config::{InputData, SegmentId, TranscriptEntry};
use std::collections::HashMap;
use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

/// Get context limit for a specific model
/// Returns 1M for Sonnet[1M] models, 200K for all others
fn get_context_limit(model_name: &str) -> u32 {
    if model_name.contains("[1m]") {
        1_000_000 // 1M for Sonnet[1M] model
    } else {
        200_000 // 200K default for all other models
    }
}

#[derive(Default)]
pub struct UsageSegment;

impl UsageSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for UsageSegment {
    fn collect(&self, input: &InputData) -> Option<SegmentData> {
        let context_used_token = if input.transcript_path == "mock_preview" {
            // Hardcoded mock data for preview
            150000
        } else {
            parse_transcript_usage(&input.transcript_path)
        };

        // Use dynamic context limit based on model
        let context_limit = get_context_limit(&input.model.display_name);
        let context_used_rate = (context_used_token as f64 / context_limit as f64) * 100.0;

        let percentage_display = if context_used_rate.fract() == 0.0 {
            format!("{:.0}%", context_used_rate)
        } else {
            format!("{:.1}%", context_used_rate)
        };

        let tokens_display = if context_used_token >= 1000 {
            let k_value = context_used_token as f64 / 1000.0;
            if k_value.fract() == 0.0 {
                format!("{}k", k_value as u32)
            } else {
                format!("{:.1}k", k_value)
            }
        } else {
            context_used_token.to_string()
        };

        let mut metadata = HashMap::new();
        metadata.insert("tokens".to_string(), context_used_token.to_string());
        metadata.insert("percentage".to_string(), context_used_rate.to_string());
        metadata.insert("limit".to_string(), context_limit.to_string());

        Some(SegmentData {
            primary: format!("{} · {} tokens", percentage_display, tokens_display),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Usage
    }
}

fn parse_transcript_usage<P: AsRef<Path>>(transcript_path: P) -> u32 {
    let file = match fs::File::open(&transcript_path) {
        Ok(file) => file,
        Err(_) => return 0,
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .collect::<Result<Vec<_>, _>>()
        .unwrap_or_default();

    for line in lines.iter().rev() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        if let Ok(entry) = serde_json::from_str::<TranscriptEntry>(line) {
            if entry.r#type.as_deref() == Some("assistant") {
                if let Some(message) = &entry.message {
                    if let Some(raw_usage) = &message.usage {
                        let normalized = raw_usage.clone().normalize();
                        return normalized.display_tokens();
                    }
                }
            }
        }
    }

    0
}
