//! Rust bindings for osv-scalibr ecosystem-aware version parsing and package extraction.

mod ffi;
pub mod manifest;
pub mod package;
pub mod version;

pub use manifest::{Manifest, ManifestError, PackageIter};
pub use package::{Package, PackageError};
pub use version::{Version, VersionError};
