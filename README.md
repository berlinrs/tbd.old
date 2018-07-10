# tbd.rs

Totally Badass Databases – in Rust.

## Proposed structure

* `tbd` is the crate that is uploaded to crates.io and used by users
  * Re-export all required types
  * Provide configuration via feature flags & similar
* `tbd-core` provides all shared code for other tbd crates
* `tbd-migrations` contains the migration toolkit for `tbd`
* `tbd-models` provides an additional macro DSL layer for creating & managing models