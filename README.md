<picture>
  <source srcset="images/osv_logo_dark-full.svg" media="(prefers-color-scheme: dark)">
  <img src="images/osv_logo_light-full.svg">
</picture>

---

# osv-rs

A collection of Rust crates for working with the [OSV (Open Source Vulnerabilities)](https://osv.dev) ecosystem — types, database management and analysis.

Official [OSV Documentation](https://google.github.io/osv.dev/) · [OSV Schema](https://ossf.github.io/osv-schema/)

## Crates

| Crate | Description |
|-------|-------------|
| [`osv-types`](osv-types/) | Rust types for OSV objects |
| [`osv-db`](osv-db/) | Download and query the OSV database locally |
| [`osv-analyzer`](osv-analyzer/) | Manifest analysis for OSV ecosystems |
