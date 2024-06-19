//! A thread-safe alternative to the `std::env` module
//!
//! # Examples
//! ```
//! use safenv::env;
//!
//! let key = "KEY";
//! env::set_var(key, "VALUE");
//! assert_eq!(env::var(key), Ok("VALUE".to_string()));
//! ```

// TODO: Update the docs

pub use std::env::{
    self, args, args_os, current_dir, current_exe, join_paths, set_current_dir, split_paths,
    temp_dir,
};
use std::ffi::{OsStr, OsString};

mod imp;

pub use imp::*;

use crate::UniversalLock;

/// Returns an iterator of (variable, value) pairs of OS strings, for all the
/// environment variables of the current process.
///
/// The returned iterator contains a snapshot of the process's environment
/// variables at the time of this invocation. Modifications to environment
/// variables afterwards will not be reflected in the returned iterator.
///
/// Note that the returned iterator will not check if the environment variables
/// are valid Unicode. If you want to panic on invalid UTF-8,
/// use the [`vars`] function instead.
///
/// # Panics
/// If the environment lock is poisoned, this function will panic.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// // We will iterate through the references to the element returned by
/// // env::vars_os();
/// for (key, value) in env::vars_os() {
///     println!("{key:?}: {value:?}");
/// }
/// ```
#[must_use]
pub fn vars_os() -> VarsOs {
    VarsOs {
        inner: ENV_MAP.u_lock().unwrap().clone().into_iter(),
    }
}

/// Returns an iterator of (variable, value) pairs of strings, for all the
/// environment variables of the current process.
///
/// The returned iterator contains a snapshot of the process's environment
/// variables at the time of this invocation. Modifications to environment
/// variables afterwards will not be reflected in the returned iterator.
///
/// # Panics
///
/// While iterating, the returned iterator will panic if any key or value in the
/// environment is not valid unicode. If this is not desired, consider using
/// [`env::vars_os()`].
///
/// If the environment lock is poisoned, this function will panic.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// // We will iterate through the references to the element returned by
/// // env::vars();
/// for (key, value) in env::vars() {
///     println!("{key}: {value}");
/// }
/// ```
///
/// [`env::vars_os()`]: vars_os
#[must_use]
pub fn vars() -> Vars {
    Vars { inner: vars_os() }
}

/// Fetches the environment variable `key` from the current process, returning
/// [`None`] if the variable isn't set or if there is another error.
///
/// It may return `None` if the environment variable's name contains
/// the equal sign character (`=`) or the NUL character.
///
/// Note that this function will not check if the environment variable
/// is valid Unicode. If you want to have an error on invalid UTF-8,
/// use the [`var`] function instead.
///
/// # Panics
/// If the environment lock is poisoned, this function will panic.

/// # Examples
///
/// ```
/// use std::env;
///
/// let key = "HOME";
/// match env::var_os(key) {
///     Some(val) => println!("{key}: {val:?}"),
///     None => println!("{key} is not defined in the environment.")
/// }
/// ```
///
/// If expecting a delimited variable (such as `PATH`), [`split_paths`]
/// can be used to separate items.
#[must_use]
pub fn var_os<K: AsRef<OsStr>>(key: K) -> Option<OsString> {
    ENV_MAP
        .u_lock()
        .unwrap()
        .get(key.as_ref())
        .map(std::borrow::ToOwned::to_owned)
}

/// Fetches the environment variable `key` from the current process.
///
/// # Errors
///
/// This function will return an error if the environment variable isn't set.
///
/// This function may return an error if the environment variable's name contains
/// the equal sign character (`=`) or the NUL character.
///
/// This function will return an error if the environment variable's value is
/// not valid Unicode. If this is not desired, consider using [`var_os`].
///
/// # Panics
/// If the environment lock is poisoned, this function will panic.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// let key = "HOME";
/// match env::var(key) {
///     Ok(val) => println!("{key}: {val:?}"),
///     Err(e) => println!("couldn't interpret {key}: {e}"),
/// }
/// ```
pub fn var<K: AsRef<OsStr>>(key: K) -> Result<String, VarError> {
    match var_os(key) {
        Some(v) => Ok(v.into_string().map_err(VarError::NotUnicode)?),
        None => Err(VarError::NotPresent),
    }
}

/// Removes an environment variable from the environment of the currently running process.
///
/// # Panics
/// If the environment lock is poisoned, this function will panic.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// let key = "KEY";
/// env::set_var(key, "VALUE");
/// assert_eq!(env::var(key), Ok("VALUE".to_string()));
///
/// env::remove_var(key);
/// assert!(env::var(key).is_err());
/// ```
pub fn remove_var<K: AsRef<OsStr>>(key: K) {
    ENV_MAP.u_lock().unwrap().remove(key.as_ref());
}

/// Sets the environment variable `key` to the value `value` for the currently running
/// process.
///
/// # Panics
/// If the environment lock is poisoned, this function will panic.
///
/// # Examples
///
/// ```
/// use std::env;
///
/// let key = "KEY";
/// env::set_var(key, "VALUE");
/// assert_eq!(env::var(key), Ok("VALUE".to_string()));
/// ```
pub fn set_var<K: AsRef<OsStr>, V: AsRef<OsStr>>(key: K, value: V) {
    ENV_MAP
        .u_lock()
        .unwrap()
        .insert(key.as_ref().to_owned(), value.as_ref().to_owned());
}

#[cfg(feature = "std")]
/// Fills the environment with the contents of the iterator `env`.
///
/// # Panics
/// If the environment lock is poisoned, this function will panic.
///
/// # Examples
///
/// ```
/// # use safenv as env;
/// let env = [("KEY", "VALUE")];
///
/// env::fill(env.into_iter());
/// assert_eq!(env::var("KEY"), Ok("VALUE".to_string()));
/// ```
pub fn fill<T: Iterator<Item = (A, B)>, A: AsRef<OsStr>, B: AsRef<OsStr>>(env: T) {
    for (key, value) in env {
        set_var(key, value);
    }
}

#[cfg(feature = "std")]
/// Fills the environment with the contents of the process' environment.
///
/// # Safety
/// This function relies on a call to [`env::vars_os()`] to get the
/// environment variables of the current process. For more information
/// about the safety of this function, see the documentation [`env::vars_os()`].
///
/// # Panics
/// If the environment lock is poisoned, this function will panic.
///
/// # Examples
///
/// ```
/// # use safenv as env;
/// unsafe { env::inherit() };
///
/// assert_eq!(env::var("CARGO_PKG_AUTHORS"), Ok("Juliette Cordor <professional@maybejules.com>".to_string()));
/// ```
///
/// [`env::vars_os()`]: std::env::vars_os
pub unsafe fn inherit() {
    for (key, value) in std::env::vars_os() {
        set_var(key, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vars() {
        set_var("KEY", "VALUE");
        assert_eq!(var("KEY"), Ok("VALUE".to_string()));

        assert_eq!(var_os("KEY"), Some(OsString::from("VALUE")));
    }
}
