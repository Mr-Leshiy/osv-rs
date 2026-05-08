//! [`Package`] — a resolved dependency discovered during a folder scan.

#![allow(clippy::module_name_repetitions)]

use osv_types::{EcosystemWithSuffix, PackageName};

/// Each entry corresponds to one package pinned in a lock file
/// (e.g. `Cargo.lock`, `uv.lock`, `package-lock.json`) and carries the name,
/// exact version, and OSV ecosystem needed to look up vulnerability records.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Package {
    /// [`Package`] name.
    pub name: PackageName,
    /// [`Package`] version string.
    pub version: String,
    /// OSV ecosystem identifier (e.g. `"npm"`, `"PyPI"`, `"crates.io"`).
    pub ecosystem: EcosystemWithSuffix,
}
