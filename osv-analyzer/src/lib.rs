//! Rust bindings for osv-scalibr ecosystem-aware version parsing and package extraction.

pub mod evaluation;
mod ffi;
pub mod package;
pub mod version;

pub use self::{
    evaluation::analyze,
    package::Package,
    version::{Version, VersionError},
};
