/// Extension trait for adding context to errors defined in this library crate.
///
/// This trait provides a standard way for modules to add contextual information
/// to their errors. Each module should implement this trait for their own
/// error types to enable context-aware error handling.
///
/// # Example for a library module's specific error type `LibModuleError`
///
/// ```ignore
/// use crate::error::ContextExt;
///
/// pub type Result<T> = std::result::Result<T, LibModuleError>;
///
/// #[derive(Debug)]
/// pub enum LibModuleError {
///     Message(String),
///     Io(std::io::Error),
///     Context {
///         context: String,
///         source: Box<dyn std::error::Error + Send + Sync>,
///     },
/// }
///
/// impl LibModuleError {
///     fn msg(msg: impl Into<String>) -> Self {
///         Self::Message(msg.into())
///     }
/// }
///
/// impl std::fmt::Display for LibModuleError {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         match self {
///             Self::Message(msg) => write!(f, "{msg}"),
///             Self::Io(e) => write!(f, "{e}"),
///             Self::Context { source, .. } => write!(f, "{source}"),
///         }
///     }
/// }
///
/// impl std::error::Error for LibModuleError {
///     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
///         match self {
///             Self::Message(_) => None,
///             Self::Io(e) => Some(e),
///             Self::Context { source, .. } => Some(source.as_ref()),
///         }
///     }
/// }
///
/// impl From<std::io::Error> for LibModuleError {
///     fn from(e: std::io::Error) -> Self {
///         Self::Io(e)
///     }
/// }
///
/// impl<T> ContextExt<T> for Option<T> {
///     type Error = LibModuleError;
///
///     fn context(self, context: impl Into<String>) -> Result<T> {
///         self.ok_or_else(|| LibModuleError::msg(context))
///     }
///
///     fn with_context<C, F>(self, f: F) -> Result<T>
///     where
///         C: Into<String>,
///         F: FnOnce() -> C,
///     {
///         self.context(f())
///     }
/// }
///
/// impl<T, E> ContextExt<T> for std::result::Result<T, E>
/// where
///     E: std::error::Error + Send + Sync + 'static,
/// {
///     type Error = LibModuleError;
///
///     fn context(self, context: impl Into<String>) -> Result<T> {
///         self.map_err(|source| LibModuleError::Context {
///             context: context.into(),
///             source: Box::new(source),
///         })
///     }
///
///     fn with_context<C, F>(self, f: F) -> Result<T>
///     where
///         C: Into<String>,
///         F: FnOnce() -> C,
///     {
///         self.context(f())
///     }
/// }
/// ```
#[allow(unused, reason = "provided for convenience")]
pub(crate) trait ContextExt<T> {
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
