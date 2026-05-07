//! Rust bindings for osv-scalibr ecosystem-aware version parsing and package extraction.

pub mod evaluation;
mod ffi;
pub mod manifest;
pub mod package;
pub mod version;

use std::collections::HashSet;

use dashmap::DashMap;
use osv_types::{OsvRecord, OsvRecordId, PackageName};

pub use self::{
    evaluation::evaluate,
    manifest::{Manifest, ManifestError, PackageIter},
    package::{ManifestPackage, PackageError},
    version::{Version, VersionError},
};

/// Manifest files (`Cargo.lock`, `npm.lock`, `yarn.lock` etc.) analyzer against OSV
/// vulnerability records.
#[derive(Debug)]
pub struct Analyzer<ManifestId> {
    /// Maps each known `Package` to the set of manifests that depend on it.
    pkg_manifests: DashMap<ManifestPackage, HashSet<ManifestId>>,
    /// Maps a package name to all known `P` versions seen across manifests.
    pkgs_by_name: DashMap<PackageName, HashSet<ManifestPackage>>,
    /// Maps a package name to all OSV record IDs that affect it.
    records_by_name: DashMap<PackageName, HashSet<OsvRecordId>>,
}

impl<ManifestId: Clone> Analyzer<ManifestId> {
    /// Creates a new, empty [`Analyzer`].
    #[must_use]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            pkg_manifests: DashMap::new(),
            pkgs_by_name: DashMap::new(),
            records_by_name: DashMap::new(),
        }
    }

    /// Registers an OSV vulnerability record and returns every [`ManifestId`] whose
    /// packages are affected by it, based on manifest files added so far.
    ///
    /// If no manifest files have been added yet, the record is still indexed so that
    /// future [`Self::add_manifest`] calls can match against it.
    pub fn add_osv_record(
        &self,
        osv_record: &OsvRecord,
    ) -> anyhow::Result<Vec<ManifestId>> {
        let mut hits = Vec::new();

        for affected_p in &osv_record.affected {
            let Some(ref package) = affected_p.package else {
                continue;
            };

            if let Some(packages) = self.pkgs_by_name.get(&package.name) {
                for p in packages.iter() {
                    if evaluate(p, affected_p)?
                        && let Some(manifests) = self.pkg_manifests.get(p)
                    {
                        for manifest_id in manifests.iter() {
                            hits.push(manifest_id.clone());
                        }
                    }
                }
            }

            self.records_by_name
                .entry(package.name.clone())
                .or_default()
                .insert(osv_record.id.clone());
        }

        Ok(hits)
    }
}
