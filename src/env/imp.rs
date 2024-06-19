use core::fmt;
use std::{collections::BTreeMap, error::Error, ffi::OsString};

#[cfg(feature = "parking_lot")]
use parking_lot::Mutex;

#[cfg(not(feature = "parking_lot"))]
use std::sync::Mutex;

pub(crate) type EnvMap = BTreeMap<OsString, OsString>;

pub(crate) static ENV_MAP: Mutex<EnvMap> = Mutex::new(BTreeMap::new());

/// The error type for operations interacting with environment variables.
/// Possibly returned from [`env::var()`].
///
/// [`env::var()`]: super::var
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VarError {
    /// The specified environment variable was not present in the current
    /// process's environment.
    NotPresent,

    /// The specified environment variable was found, but it did not contain
    /// valid unicode data. The found data is returned as a payload of this
    /// variant.
    NotUnicode(OsString),
}

impl fmt::Display for VarError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            VarError::NotPresent => write!(f, "environment variable not found"),
            VarError::NotUnicode(ref s) => {
                write!(f, "environment variable was not valid unicode: {s:?}")
            }
        }
    }
}

impl Error for VarError {
    fn description(&self) -> &str {
        match *self {
            VarError::NotPresent => "environment variable not found",
            VarError::NotUnicode(..) => "environment variable was not valid unicode",
        }
    }
}

#[derive(Debug)]
/// An iterator over a snapshot of the environment variables of this process.
///
/// This structure is created by [`env::vars_os()`]. See its documentation for more.
///
/// [`env::vars_os()`]: super::vars_os
pub struct VarsOs {
    pub(crate) inner: std::collections::btree_map::IntoIter<std::ffi::OsString, std::ffi::OsString>,
}

impl Iterator for VarsOs {
    type Item = (OsString, OsString);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, v)| (k.clone(), v.clone()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

#[derive(Debug)]
/// An iterator over a snapshot of the environment variables of this process.
///
/// This structure is created by [`env::vars()`]. See its documentation for more.
///
/// [`env::vars()`]: super::vars
pub struct Vars {
    pub(crate) inner: VarsOs,
}

impl Iterator for Vars {
    type Item = (String, String);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner
            .next()
            .map(|(k, v)| (k.into_string().unwrap(), v.into_string().unwrap()))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}
