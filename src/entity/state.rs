use std::any::Any;

/// The trait that is implemented by the object that represents the State of an
/// Entity. It exposes methods that enable dynamic typing of any `'static` type
/// through runtime reflection via the `Any` trait, so that you can downcast this
/// trait to its original concrete type.
/// For more information about type downcasting and dynamic typing please refer
/// to the [std documentation](https://doc.rust-lang.org/beta/std/any/index.html).
pub trait State {
    /// Gets a reference to self via the Any trait, used to emulate dynamic
    /// typing and downcast this trait to its concrete type.
    fn as_any(&self) -> &dyn Any;

    /// Gets a mutable reference to self via the Any trait, used to emulate dynamic
    /// typing and downcast this trait to its concrete type.
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
