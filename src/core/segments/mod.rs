pub mod directory;
pub mod git;
pub mod model;
pub mod quota;
pub mod update;
pub mod usage;

use crate::config::{InputData, SegmentId};
use std::collections::HashMap;

// New Segment trait for data collection only
pub trait Segment {
    fn collect(&self, input: &InputData) -> Option<SegmentData>;
    fn id(&self) -> SegmentId;
}

#[derive(Debug, Clone)]
pub struct SegmentData {
    pub primary: String,
    pub secondary: String,
    pub metadata: HashMap<String, String>,
}

// Re-export all segment types
pub use directory::DirectorySegment;
pub use git::GitSegment;
pub use model::ModelSegment;
pub use quota::QuotaSegment;
pub use update::UpdateSegment;
pub use usage::UsageSegment;
