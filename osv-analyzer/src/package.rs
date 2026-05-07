//! [`Package`] — a single package extracted from a manifest file.

#![allow(clippy::module_name_repetitions)]

use std::{
    ffi::{CStr, c_char, c_void},
    ptr,
};

use osv_types::{EcosystemWithSuffix, PackageName};
use thiserror::Error;

use crate::ffi;

/// A single package extracted from a manifest file.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Package {
    /// Package name.
    pub name: PackageName,
    /// Package version string.
    pub version: String,
    /// OSV ecosystem identifier (e.g. `"npm"`, `"PyPI"`, `"crates.io"`).
    pub ecosystem: EcosystemWithSuffix,
}

/// Errors returned when constructing a [`Package`].
#[derive(Debug, Error)]
pub enum PackageError {
    /// A C field accessor returned an error.
    #[error("package field error: {0}")]
    Field(String),
    /// The ecosystem string returned by the extractor is not a known OSV ecosystem.
    #[error("unknown ecosystem: {0}")]
    UnknownEcosystem(#[from] strum::ParseError),
}

impl Package {
    /// Constructs a [`Package`] by reading the fields of the package at `idx`
    /// from the Go-managed manifest identified by `manifest`.
    ///
    /// # Errors
    ///
    /// Returns [`PackageError`] if `idx` is out of range or any field accessor fails.
    pub(crate) fn new(
        manifest: usize,
        idx: usize,
    ) -> Result<Self, PackageError> {
        let name = read_field(manifest, idx, manifest_package_name)?;
        let version = read_field(manifest, idx, manifest_package_version)?;
        let ecosystem = read_field(manifest, idx, manifest_package_ecosystem)?.parse()?;
        Ok(Self {
            name,
            version,
            ecosystem,
        })
    }
}

#[allow(clippy::missing_docs_in_private_items)]
unsafe extern "C" {
    fn manifest_package_name(
        manifest: usize,
        idx: usize,
        out: *mut *mut c_char,
    ) -> *mut c_char;
    fn manifest_package_version(
        manifest: usize,
        idx: usize,
        out: *mut *mut c_char,
    ) -> *mut c_char;
    fn manifest_package_ecosystem(
        manifest: usize,
        idx: usize,
        out: *mut *mut c_char,
    ) -> *mut c_char;
}

/// Reads one heap-allocated string from a C field accessor.
///
/// Frees both the error and value pointers. Returns `Err` on error.
fn read_field(
    manifest: usize,
    idx: usize,
    f: unsafe extern "C" fn(usize, usize, *mut *mut c_char) -> *mut c_char,
) -> Result<String, PackageError> {
    unsafe {
        let mut out: *mut c_char = ptr::null_mut();
        let err = f(manifest, idx, &raw mut out);
        let msg = CStr::from_ptr(err).to_string_lossy().into_owned();
        ffi::free(err.cast::<c_void>());
        if !msg.is_empty() {
            return Err(PackageError::Field(msg));
        }
        let s = CStr::from_ptr(out).to_string_lossy().into_owned();
        ffi::free(out.cast::<c_void>());
        Ok(s)
    }
}
