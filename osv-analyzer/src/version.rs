//! Ecosystem-aware version parsing backed by the osv-scalibr C library.

#![allow(clippy::module_name_repetitions)]

use std::{
    cmp::Ordering,
    ffi::{CStr, CString, NulError, c_char, c_int, c_void},
};

use thiserror::Error;

use crate::ffi;

/// An opaque handle to an ecosystem-aware parsed version.
///
/// Create one with [`Version::new`] and compare instances with [`PartialOrd`].
/// The underlying Go handle is released automatically when the value is dropped.
#[derive(Debug)]
pub struct Version(VersionHandle);

/// Errors returned by [`Version::new`].
#[derive(Debug, Error)]
pub enum VersionError {
    /// The string contained an interior null byte and cannot be passed to the C library.
    #[error("invalid string: {0}")]
    InvalidString(#[from] NulError),
    /// The C library rejected the version string or ecosystem name.
    #[error("parse error: {0}")]
    Parse(String),
}

impl Version {
    /// Parses `str` as a version in `ecosystem` (e.g. `"npm"`, `"PyPI"`).
    ///
    /// # Errors
    ///
    /// - [`VersionError::InvalidString`]
    /// - [`VersionError::Parse`]
    pub fn new(
        str: &str,
        ecosystem: &str,
    ) -> Result<Self, VersionError> {
        let c_str = CString::new(str)?;
        let c_ecosystem = CString::new(ecosystem)?;
        let mut handle: VersionHandle = 0;
        unsafe {
            let err = version_new(c_str.as_ptr(), c_ecosystem.as_ptr(), &raw mut handle);
            let msg = CStr::from_ptr(err).to_string_lossy().into_owned();
            ffi::free(err.cast::<c_void>());
            if msg.is_empty() {
                Ok(Self(handle))
            } else {
                Err(VersionError::Parse(msg))
            }
        }
    }
}

impl Drop for Version {
    fn drop(&mut self) {
        unsafe { version_free(self.0) }
    }
}

impl PartialEq for Version {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.partial_cmp(other) == Some(Ordering::Equal)
    }
}

impl PartialOrd for Version {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        let mut result: c_int = 0;
        unsafe {
            let err = version_cmp(self.0, other.0, &raw mut result);
            let msg = CStr::from_ptr(err).to_string_lossy().into_owned();
            ffi::free(err.cast::<c_void>());
            if msg.is_empty() {
                Some(result.cmp(&0))
            } else {
                None
            }
        }
    }
}

/// Opaque integer key that identifies a Go-managed version object across the cgo
/// boundary.
type VersionHandle = usize;

#[allow(clippy::missing_docs_in_private_items)]
unsafe extern "C" {
    fn version_new(
        str: *const c_char,
        ecosystem: *const c_char,
        out: *mut VersionHandle,
    ) -> *mut c_char;
    fn version_cmp(
        a: VersionHandle,
        b: VersionHandle,
        result: *mut c_int,
    ) -> *mut c_char;
    fn version_free(handle: VersionHandle);
}
