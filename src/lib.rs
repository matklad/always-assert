//! Recoverable assertions, inspired by [the use of `assert()` in
//! SQLite](https://www.sqlite.org/assert.html).
//!
//! `never!` and `always!` return the actual value of the condition if
//! `debug_assertions` are disabled.
//!
//! Use them when terminating on assertion failure is worse than continuing.
//!
//! One example would be a critical application like a database:
//!
//! ```ignore
//! use always_assert::never;
//!
//! fn apply_transaction(&mut self, tx: Transaction) -> Result<(), TransactionAborted> {
//!     let delta = self.compute_delta(&tx);
//!
//!     if never!(!self.check_internal_invariant(&delta)) {
//!         // Ok, something in this transaction messed up our internal state.
//!         // This really shouldn't be happening, and this signifies a bug.
//!         // Luckily, we can recover by just rejecting the transaction.
//!         return abort_transaction(tx);
//!     }
//!     self.commit(delta);
//!     Ok(())
//! }
//! ```
//!
//! Another example is assertions about non-critical functionality in usual apps
//!
//! ```ignore
//! use always_assert::never;
//!
//! let english_message = "super app installed!"
//! let mut local_message = localize(english_message);
//! if never!(local_message.is_empty(), "missing localization for {}", english_message) {
//!     // We localized all the messages but this one slipper through the cracks?
//!     // Better to show the english one then than to fail outright;
//!     local_message = english_message;
//! }
//! println!("{}", local_message);
//! ```

/// Asserts that the condition is always true and returns its actual value.
///
/// If the condition is true does nothing and and evaluates to true.
///
/// If the condition is false:
/// * panics if `force` feature or `debug_assertions` are enabled,
/// * logs an error if `log` feature is enabled,
/// * evaluates to false.
///
/// Accepts `format!` style arguments.
#[macro_export]
macro_rules! always {
    ($cond:expr) => {
        $crate::always!($cond, "assertion failed: {}", stringify!($cond))
    };

    ($cond:expr, $fmt:literal $($arg:tt)*) => {{
        let cond = $cond;
        if cfg!(debug_assertions) || $crate::__FORCE {
            assert!(cond, $fmt $($arg)*);
        }
        if !cond {
            $crate::__log_error!($fmt $($arg)*);
        }
        cond
    }}
}

/// Asserts that the condition is never true and returns its actual value.
///
/// If the condition is false does nothing and and evaluates to false.
///
/// If the condition is true:
/// * panics if `force` feature or `debug_assertions` are enabled,
/// * logs an error if `log` feature is enabled,
/// * evaluates to true.
///
/// Accepts `format!` style arguments.
#[macro_export]
macro_rules! never {
    ($cond:expr) => {
        !$crate::always!(!$cond)
    };

    ($cond:expr, $fmt:literal $($arg:tt)*) => {
        !$crate::always!(!$cond, $fmt $($arg)*)
    }
}

#[cfg(feature = "log")]
#[doc(hidden)]
pub use log::error as __log_error;

#[cfg(not(feature = "log"))]
#[doc(hidden)]
#[macro_export]
macro_rules! __log_error {
    ($($tt:tt)*) => {};
}

#[doc(hidden)]
pub const __FORCE: bool = cfg!(feature = "force");