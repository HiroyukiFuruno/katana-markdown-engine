mod resolver;
mod types;

pub use resolver::MetadataResolver;
pub use types::{
    ContextAnchor, MetadataDocument, MetadataEntry, MetadataTarget, TargetResolution,
    TargetResolutionKind, UnresolvedTarget,
};
