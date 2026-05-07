//! [`osv_analyzer::Manifest`] tests

#![allow(clippy::unwrap_used)]

use osv_analyzer::{Manifest, Package};
use test_case::test_case;

fn pkg(
    name: &str,
    version: &str,
    ecosystem: &str,
) -> Package {
    Package {
        name: name.to_owned(),
        version: version.to_owned(),
        ecosystem: ecosystem.to_owned(),
    }
}

#[test_case(
    "tests/testdata/manifests/Cargo.lock",
    "Cargo.lock"
    =>
    vec![
        pkg("async-trait", "0.1.89", "crates.io"),
        pkg("zstd-sys", "2.0.16+zstd.1.5.7", "crates.io")
    ]
)]
#[test_case(
    "tests/testdata/manifests/uv.lock",
    "uv.lock"
    =>
    vec![
        pkg("anthropic", "0.84.0", "PyPI"),
        pkg("anyio", "4.12.1", "PyPI")
    ]
)]
fn extract_packages(
    manifest_path: &str,
    ecosystem: &str,
) -> Vec<Package> {
    let data = std::fs::read(manifest_path).unwrap();
    let manifest = Manifest::extract(ecosystem, &data).unwrap();
    manifest.iter().collect::<Result<Vec<_>, _>>().unwrap()
}
