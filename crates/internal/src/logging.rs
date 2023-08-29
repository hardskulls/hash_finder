/// Trait used to look at the error without consuming result.
/// Useful for logging.
///
/// # Example
/// ```
/// use internal::logging::PeekErr;
///
/// let num = "nothing".parse::<u16>().peek_err(|e| log::error!("error: {e:?}"));
/// ```
pub trait PeekErr<E, R> {
    fn peek_err(self, f: impl FnOnce(&E)) -> R;
}

impl<T, E> PeekErr<E, Result<T, E>> for Result<T, E> {
    fn peek_err(self, f: impl FnOnce(&E)) -> Result<T, E> {
        if let Err(e) = &self {
            f(e)
        }
        self
    }
}
