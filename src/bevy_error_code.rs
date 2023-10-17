#![allow(clippy::zero_prefixed_literal)]

use std::fmt::Display;

use crate::ErrorCode;

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum BevyErrorCode {
    CollidingQueryAccess = 0001,
    CollidingResourceAccess = 0002,
    CommandOnDespawnedEntity = 0003,
    MissingHierarchyInheritedComponent = 0004,
    TooManyFontAtlases = 0005,
}

impl Display for BevyErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B{:0>4}", *self as u16)
    }
}

impl ErrorCode for BevyErrorCode {
    fn url(&self) -> Option<String> {
        Some(format!(
            "https://bevyengine.org/learn/errors/#b{:0>4}",
            *self as u16
        ))
    }
}
