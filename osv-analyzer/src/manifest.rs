//! [`Manifest`] — ecosystem-aware package extraction backed by osv-scalibr.

#![allow(clippy::module_name_repetitions)]

use std::ffi::{CStr, CString, NulError, c_char, c_void};

use thiserror::Error;

use crate::{
    ffi,
    package::{ManifestPackage, PackageError},
};

/// An opaque handle to a Go-managed list of extracted packages.
///
/// Create one with [`Manifest::extract`] and iterate with [`Manifest::iter`].
/// The underlying Go handle is released automatically when the value is dropped.
#[derive(Debug)]
pub struct Manifest(ManifestHandle);

/// Errors returned by [`Manifest::extract`].
#[derive(Debug, Error)]
pub enum ManifestError {
    /// The ecosystem string contained an interior null byte.
    #[error("invalid string: {0}")]
    InvalidString(#[from] NulError),
    /// The C library failed to extract packages from the provided bytes.
    #[error("extraction error: {0}")]
    Extraction(String),
}

impl Manifest {
    /// Parses `data` using the extractor for `ecosystem`
    /// (e.g. `"npm"`, `"PyPI"`, `"crates.io"`).
    ///
    /// Returns an empty list for unknown ecosystems.
    ///
    /// # Errors
    ///
    /// - [`ManifestError::InvalidString`] if `ecosystem` contains an interior null byte.
    /// - [`ManifestError::Extraction`] if the extractor fails to parse `data`.
    pub fn extract(
        ecosystem: &str,
        data: &[u8],
    ) -> Result<Self, ManifestError> {
        let c_ecosystem = CString::new(ecosystem)?;
        let mut handle: ManifestHandle = 0;
        unsafe {
            let err = manifest_parse(
                c_ecosystem.as_ptr(),
                data.as_ptr(),
                data.len(),
                &raw mut handle,
            );
            let msg = CStr::from_ptr(err).to_string_lossy().into_owned();
            ffi::free(err.cast::<c_void>());
            if msg.is_empty() {
                Ok(Self(handle))
            } else {
                Err(ManifestError::Extraction(msg))
            }
        }
    }

    /// Returns the number of packages in the list.
    #[must_use]
    pub fn len(&self) -> usize {
        let mut n: usize = 0;
        unsafe {
            // manifest_packages_len is infallible for a valid handle; free the
            // error string without inspecting it.
            let err = manifest_packages_len(self.0, &raw mut n);
            ffi::free(err.cast::<c_void>());
        }
        n
    }

    /// Returns `true` if the list contains no packages.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns an iterator over the packages in this manifest.
    #[allow(clippy::iter_without_into_iter)]
    #[must_use]
    pub fn iter(&self) -> PackageIter<'_> {
        PackageIter {
            manifest: self,
            idx: 0,
        }
    }
}

impl Drop for Manifest {
    fn drop(&mut self) {
        unsafe { manifest_free(self.0) }
    }
}

/// An iterator over the [`Package`]s in a [`Manifest`].
pub struct PackageIter<'a> {
    /// The source manifest.
    manifest: &'a Manifest,
    /// Current index.
    idx: usize,
}

impl Iterator for PackageIter<'_> {
    type Item = Result<ManifestPackage, PackageError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.manifest.len() {
            return None;
        }
        let idx = self.idx;
        self.idx = self.idx.saturating_add(1);
        Some(ManifestPackage::new(self.manifest.0, idx))
    }
}

/// Opaque integer key that identifies a Go-managed package list across the cgo boundary.
type ManifestHandle = usize;

#[allow(clippy::missing_docs_in_private_items)]
unsafe extern "C" {
    fn manifest_parse(
        ecosystem: *const c_char,
        data: *const u8,
        data_len: usize,
        out: *mut ManifestHandle,
    ) -> *mut c_char;
    fn manifest_packages_len(
        manifest: ManifestHandle,
        out: *mut usize,
    ) -> *mut c_char;
    fn manifest_free(manifest: ManifestHandle);
}
