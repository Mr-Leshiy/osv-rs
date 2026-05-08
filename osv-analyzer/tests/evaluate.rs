//! [`osv_analyzer::evaluate`] tests
//!
//! OSV records used by these tests live in `tests/testdata/osv_records/`.
//! To add a new record, use the fetch script:
//!
//! ```sh
//! ./scripts/fetch_osv_record.sh <ecosystem> <record-id> osv-analyzer/tests/testdata/osv_records
//! # e.g.
//! ./scripts/fetch_osv_record.sh crates.io RUSTSEC-2025-0112 osv-analyzer/tests/testdata/osv_records
//! ```

#![allow(clippy::unwrap_used)]

use osv_analyzer::{ManifestPackage, evaluate};
use osv_types::OsvRecord;
use test_case::test_case;

fn pkg(
    name: &str,
    version: &str,
    ecosystem: &str,
) -> ManifestPackage {
    ManifestPackage {
        name: name.to_owned(),
        version: version.to_owned(),
        ecosystem: ecosystem.parse().unwrap(),
    }
}

#[test_case(pkg("wasmtime", "38.0.0", "crates.io"), "RUSTSEC-2025-0112" => true)]
#[test_case(pkg("wasmtime", "38.0.3", "crates.io"), "RUSTSEC-2025-0112" => false)]
#[test_case(pkg("tokio", "38.0.0", "crates.io"), "RUSTSEC-2025-0112" => false)]
#[test_case(pkg("github.com/Tencent/WeKnora", "0.2.0", "Go"), "GHSA-8rf9-c59g-f82f" => true)]
#[test_case(pkg("github.com/Tencent/WeKnora", "0.3.0", "Go"), "GHSA-8rf9-c59g-f82f" => false)]
fn evaluate_against_osv_record(
    p: ManifestPackage,
    osv_record_id: &str,
) -> bool {
    let path = format!("tests/testdata/osv_records/{osv_record_id}.json");
    let data = std::fs::read_to_string(path).unwrap();
    let record: OsvRecord = serde_json::from_str(&data).unwrap();
    record
        .affected
        .iter()
        .any(|affected| evaluate(&p, affected).unwrap())
}
