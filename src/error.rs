use std::fmt::{self, Debug, Display};

pub trait Any: std::error::Error {
    /// Gets a reference to self via the Any trait, used to emulate dynamic
    /// typing and downcast this trait to its concrete type.
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Represents any possible error.
///
/// This enum allows to encode the errors that can be raised by this library as
/// well as allow to encode a custom error, that will need to be propagated via
/// the APIs exposed by this library, which originates in the user's code.
#[derive(Debug)]
pub enum Error {
    /// The most generic and possibly useless type of error, to be raised when
    /// no other information is available.
    Unknown,
    /// The Code variant allows to encode errors as simple signed integers.
    Code(i32),
    /// The Message variant allows to encode the error as a string.
    Message(String),
    /// The Any variant allows to encode any type of error with performance costs
    /// due to the heap allocations, and type erasure.
    ///
    /// # Example
    /// ```
    /// use std::fmt;
    ///
    /// #[derive(Debug)]
    /// enum MyError {
    ///     Audio,
    ///     Game
    /// }
    ///
    /// impl fmt::Display for MyError {
    ///     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///         write!(f, "{:?}", self)
    ///     }
    /// }
    ///
    /// impl std::error::Error for MyError {}
    ///
    /// impl semeion::error::Any for MyError {
    ///     fn as_any(&self) -> &dyn std::any::Any {
    ///         self
    ///     }
    /// }
    ///
    /// // we can get a `semeion::Error` from any of the public APIs
    /// let err = semeion::Error::with_err(MyError::Audio);
    ///
    /// // and downcast it back to its original concrete type with
    /// if let semeion::Error::Any(err) = err {
    ///     let my_err = err.as_any().downcast_ref::<MyError>().unwrap();
    /// }
    /// ```
    Any(Box<dyn Any>),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unknown => write!(f, "Error unknown"),
            Self::Code(code) => write!(f, "{}", code),
            Self::Message(message) => write!(f, "{}", message),
            Self::Any(err) => write!(f, "{}", err),
        }
    }
}

impl std::error::Error for Error {}

impl Error {
    /// Constructs a new Error with the given message.
    pub fn with_message(message: impl Display) -> Self {
        Self::Message(message.to_string())
    }

    /// Constructs a new Error with the given custom value.
    pub fn with_err(err: impl Any + 'static) -> Self {
        Self::Any(Box::new(err))
    }
}
