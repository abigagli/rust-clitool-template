/// Extension trait for adding context to errors.
///
/// This trait provides a standard way for modules to add contextual information
/// to their errors. Each module should implement this trait for their own
/// error types to enable context-aware error handling.
///
/// # Example
///
/// ```ignore
/// impl<T> ContextExt<T> for Option<T> {
///     fn context(self, context: impl Into<String>) -> Result<T> {
///         self.ok_or_else(|| MyError::msg(context))
///     }
///     // ... with_context implementation
/// }
/// ```
pub trait ContextExt<T> {
    /// The error type that will be returned.
    type Error;

    /// Adds context to an error.
    fn context(self, context: impl Into<String>) -> Result<T, Self::Error>;

    /// Adds context to an error using a lazy function.
    ///
    /// This is useful when constructing the context message is expensive
    /// and should only be done if an error actually occurs.
    fn with_context<C, F>(self, f: F) -> Result<T, Self::Error>
    where
        C: Into<String>,
        F: FnOnce() -> C;
}
