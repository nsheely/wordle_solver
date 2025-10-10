//! Embedded word lists
//!
//! Word lists compiled into the binary at build time.

// Include generated word lists from build script
include!(concat!(env!("OUT_DIR"), "/answers.rs"));
include!(concat!(env!("OUT_DIR"), "/allowed.rs"));
