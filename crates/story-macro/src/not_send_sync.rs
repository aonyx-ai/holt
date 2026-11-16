//! Helpers that prove a type is neither `Send` nor `Sync`

/// Compiles for a type only if the type is not `Send`
///
/// A type that is `Send` makes the call ambiguous, because two impls then apply.
pub(crate) trait AmbiguousIfSend<A> {
    fn assert_not_send() {}
}

impl<T: ?Sized> AmbiguousIfSend<()> for T {}

impl<T: ?Sized + Send> AmbiguousIfSend<u8> for T {}

/// Compiles for a type only if the type is not `Sync`
///
/// A type that is `Sync` makes the call ambiguous, because two impls then apply.
pub(crate) trait AmbiguousIfSync<A> {
    fn assert_not_sync() {}
}

impl<T: ?Sized> AmbiguousIfSync<()> for T {}

impl<T: ?Sized + Sync> AmbiguousIfSync<u8> for T {}
