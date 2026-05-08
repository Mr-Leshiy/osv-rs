# osv-types

Rust types for [OSV (Open Source Vulnerabilities)](https://osv.dev) objects.

## Overview

`osv-types` provides a complete, strongly-typed representation of the [OSV schema spec](https://ossf.github.io/osv-schema/), including `OsvRecord` and all nested types — affected packages, version ranges, severity ratings, references, and more. All types implement `serde::Serialize` / `serde::Deserialize` and are ready for use with any JSON source.

## Usage

```rust
use osv_types::OsvRecord;

let record: OsvRecord = serde_json::from_str(json)?;
println!("{}: {:?}", record.id, record.summary);
```
