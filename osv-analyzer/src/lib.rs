//! Rust bindings for osv-scalibr ecosystem-aware version parsing and package extraction.

pub mod evaluation;
mod ffi;
pub mod manifest;
pub mod package;
pub mod version;

pub use self::{
    evaluation::evaluate,
    manifest::{Manifest, ManifestError, PackageIter},
    package::{ManifestPackage, PackageError},
    version::{Version, VersionError},
};
