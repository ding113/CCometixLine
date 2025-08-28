use super::{Segment, SegmentData};
use crate::config::{InputData, ModelConfig, SegmentId};
use std::collections::HashMap;

#[derive(Default)]
pub struct ModelSegment;

impl ModelSegment {
    pub fn new() -> Self {
        Self
    }
}

impl Segment for ModelSegment {
    fn collect(&self, input: &InputData) -> Option<SegmentData> {
        let mut metadata = HashMap::new();
        metadata.insert("model_id".to_string(), input.model.id.clone());
        metadata.insert("display_name".to_string(), input.model.display_name.clone());

        Some(SegmentData {
            primary: self.format_model_name(&input.model.id, &input.model.display_name),
            secondary: String::new(),
            metadata,
        })
    }

    fn id(&self) -> SegmentId {
        SegmentId::Model
    }
}

impl ModelSegment {
    fn format_model_name(&self, id: &str, display_name: &str) -> String {
        let model_config = ModelConfig::load();

        // Try to get display name from external config first
        if let Some(config_name) = model_config.get_display_name(id) {
            config_name
        } else {
            // Fallback to Claude Code's official display_name for unrecognized models
            display_name.to_string()
        }
    }
}
