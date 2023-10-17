use crate::ErrorCode;

pub struct BevyError<C: ErrorCode> {
    code: C,
    summary: String,
}

impl<C: ErrorCode> BevyError<C> {
    pub fn new(code: C, summary: String) -> Self {
        BevyError { code, summary }
    }

    /// Abort program execution and emit the error as panic message.
    pub fn panic_with_error(&self) {
        panic!("{}", self.create_message("error"));
    }

    /// Log the error as warning to inform the user about the problem.
    pub fn log_as_warning(&self) {
        println!("{}", self.create_message("warning"));
    }

    /// Assemble all information we have about the error into a user-friendly message.
    fn create_message(&self, message_type: &'static str) -> String {
        let mut message = format!("{message_type}[{}]: {}", self.code, self.summary);

        if let Some(url) = self.code.url() {
            message += &format!(
                "\n  = note: for more information about this {message_type}, visit <{url}>"
            );
        }

        message
    }
}
