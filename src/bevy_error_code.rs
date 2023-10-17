#![allow(clippy::zero_prefixed_literal)]

#[repr(u16)]
pub enum BevyErrorCode {
    CollidingQueryAccess = 0001,
    CollidingResourceAccess = 0002,
    CommandOnDespawnedEntity = 0003,
    MissingHierarchyInheritedComponent = 0004,
    TooManyFontAtlases = 0005,
}
