#!/usr/bin/env bash
set -euo pipefail

OSV_STORAGE_URL="https://storage.googleapis.com/osv-vulnerabilities"

usage() {
    echo "Usage: $0 <ecosystem> <record_name> <output_dir>"
    echo "  ecosystem   - e.g. PyPI, npm, crates.io"
    echo "  record_name - e.g. GHSA-1234-5678-9abc"
    echo "  output_dir  - directory where the JSON file will be saved"
    exit 1
}

[[ $# -ne 3 ]] && usage

ecosystem="$1"
record_name="$2"
output_dir="$3"
url="${OSV_STORAGE_URL}/${ecosystem}/${record_name}.json"

curl --fail --show-error --location --output "${output_dir}/${record_name}.json" "$url"
