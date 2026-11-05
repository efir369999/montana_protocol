#![cfg(windows)]

//! The Windows platform's own file guarantees, kept in a crate that may use the platform's API
//! because the protocol crates forbid unsafe code: an owner-only file of details (K1), and the
//! directory flush a publication stands on.

pub mod directory;
pub mod nt;
pub mod private_log;
