pub trait ErrorCode {
    /// The identifier of this error code.
    ///
    /// This is usually a character followed by a number.
    fn code(&self) -> String;

    /// A link to a webpage containing more information about the error, if available.
    fn url(&self) -> Option<String> {
        None
    }
}
