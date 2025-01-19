//! A Rust library for audio effects and processing.
//!
//! This library provides a set of modules that are capable of real-time audio processing. Python
//! bindings are also provided, but not all the Rust features are available in Python.

pub mod filter;
pub mod effects;
pub mod buffer_view;
mod utilities;
