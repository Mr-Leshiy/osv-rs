//! Resolved dependency entries produced by a folder scan.

#![allow(clippy::module_name_repetitions)]

use osv_types::{EcosystemWithSuffix, PackageName};

/// A single dependency pinned in a project's lock file.
///
/// Produced by [`crate::Extractor`] after scanning a folder. Each value
/// represents one package at an exact version inside a specific OSV ecosystem,
/// which is the minimum information needed to query vulnerability records.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Package {
    /// The package name as it appears in the ecosystem registry.
    pub name: PackageName,
    /// The exact pinned version string from the lock file.
    pub version: String,
    /// The OSV ecosystem this package belongs to (e.g. `"crates.io"`, `"PyPI"`, `"npm"`).
    pub ecosystem: EcosystemWithSuffix,
}
