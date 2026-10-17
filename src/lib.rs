//! The `.hnl` source of the `dedent` library.
//!
//! Exported as a const so a host crate can `include` it into its module
//! chain without owning the source (ADR-0128 ".hnl ownership").

/// The handl source of `dedent.hnl`.
pub const DEDENT_HNL: &str = include_str!("dedent.hnl");
