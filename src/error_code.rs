use std::fmt::Display;

pub trait ErrorCode: Display {
    /// A link to a webpage containing more information about the error, if available.
    fn url(&self) -> Option<String> {
        None
    }
}
