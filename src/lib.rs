//! A thread-safe alternative to the `std::env` module
//!
//! # Examples
//! ```
//! use safenv as env;
//!
//! let key = "KEY";
//! env::set_var(key, "VALUE");
//! assert_eq!(env::var(key), Ok("VALUE".to_string()));
//! ```

#![warn(clippy::all, clippy::pedantic, missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

use core::ops::DerefMut;

pub mod env;

pub use env::*;

trait UniversalLock {
    type Target;
    type Lock<'a>: DerefMut<Target = Self::Target>
    where
        Self::Target: 'a,
        Self: 'a;

    type InfallibleError<'a>
    where
        Self::Target: 'a,
        Self: 'a;
    type FallibleError<'a>
    where
        Self::Target: 'a,
        Self: 'a;

    fn u_lock(&self) -> Result<Self::Lock<'_>, Self::InfallibleError<'_>>;

    #[allow(dead_code)]
    fn u_try_lock(&self) -> Result<Self::Lock<'_>, Self::FallibleError<'_>>;
}

impl<T> UniversalLock for std::sync::Mutex<T> {
    type Target = T;
    type Lock<'a> = std::sync::MutexGuard<'a, T> where T: 'a;

    type InfallibleError<'a> = std::sync::PoisonError<Self::Lock<'a>> where T: 'a;
    type FallibleError<'a> = std::sync::TryLockError<Self::Lock<'a>> where T: 'a;

    fn u_lock(&self) -> Result<Self::Lock<'_>, Self::InfallibleError<'_>> {
        std::sync::Mutex::lock(self)
    }

    fn u_try_lock(&self) -> Result<Self::Lock<'_>, Self::FallibleError<'_>> {
        std::sync::Mutex::try_lock(self)
    }
}

#[cfg(feature = "parking_lot")]
impl<T> UniversalLock for parking_lot::Mutex<T> {
    type Target = T;
    type Lock<'a> = parking_lot::MutexGuard<'a, T> where T: 'a;

    type InfallibleError<'a> = () where T: 'a;
    type FallibleError<'a> = () where T: 'a;

    fn u_lock(&self) -> Result<Self::Lock<'_>, Self::InfallibleError<'_>> {
        Ok(parking_lot::Mutex::lock(self))
    }

    fn u_try_lock(&self) -> Result<Self::Lock<'_>, Self::FallibleError<'_>> {
        parking_lot::Mutex::try_lock(self).ok_or(())
    }
}
