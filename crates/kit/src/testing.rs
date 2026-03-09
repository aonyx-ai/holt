//! Test helpers for Leptos reactive logic.
//!
//! Leptos signals, callbacks, and contexts require a reactive [`Owner`] to function. Outside of a
//! running Leptos application there is no owner on the current thread, so tests must create one
//! manually. These helpers reduce that boilerplate so tests can focus on the logic under test.
//!
//! See the [`Owner`] docs for details on how reactive ownership works:
//! <https://docs.rs/leptos/latest/leptos/reactive/owner/struct.Owner.html>

use leptos::prelude::*;

/// Run a closure inside a Leptos reactive scope.
///
/// Creates a temporary [`Owner`] and executes `f` within it. Any reactive primitives ([`RwSignal`],
/// [`Signal`], [`Callback`], etc.) created inside `f` will be properly tracked and disposed when
/// the scope ends.
///
/// Use this whenever a test needs to create or interact with reactive state without spinning up a
/// full Leptos application.
///
/// ```
/// use holt_kit::testing::reactive_scope;
/// use leptos::prelude::*;
///
/// reactive_scope(|| {
///     let count = RwSignal::new(0);
///     count.set(1);
///     assert_eq!(count.get(), 1);
/// });
/// ```
pub fn reactive_scope<R>(f: impl FnOnce() -> R) -> R {
    let owner = Owner::new();
    owner.with(f)
}

/// Create a callback that captures its argument into a readable signal.
///
/// Returns `(callback, signal)`. Each call to `callback.run(value)` stores `Some(value)` in the
/// signal, which can then be read with `signal.get()`.
///
/// This replaces the common test pattern of manually wiring up an `RwSignal<Option<T>>` and a
/// `Callback` that sets it.
///
/// Must be called inside a [`reactive_scope`].
///
/// ```
/// use holt_kit::testing::{reactive_scope, track_callback};
///
/// reactive_scope(|| {
///     let (cb, last) = track_callback::<i32>();
///     assert_eq!(last.get(), None);
///     cb.run(42);
///     assert_eq!(last.get(), Some(42));
/// });
/// ```
pub fn track_callback<T>() -> (Callback<T>, RwSignal<Option<T>>)
where
    T: Send + Sync + 'static,
{
    let value = RwSignal::new(None::<T>);
    let cb = Callback::new(move |v: T| value.set(Some(v)));
    (cb, value)
}
