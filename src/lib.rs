//! # Crazy Train
//!
//! **Crazy Train** is a Rust library designed for randomized and fuzz execution of command-line
//! interfaces (CLIs). It helps discover unforeseen sequences of steps and parameters that can lead
//! to unexpected errors. This library facilitates reproducible test plan runs, ensuring that the
//! command-line interface behaves as expected under various scenarios.
//!
//! ## Features
//!
//! - **Randomized Execution**: Execute commands with random parameters and sequences to explore
//!   unexpected behaviors.
//! - **Fuzz Testing**: Identify edge cases and potential bugs by fuzzing input to the command line.
//! - **Reproducible Tests**: Create a test plan that can be repeated to ensure consistency in test
//!   results.
//! - **Error Discovery**: Capture and report unforeseen errors encountered during execution.
//!
//! ## Getting Started
//!
//! To start using Crazy Train in your project, add it to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! crazy-train = "0.1.0"  // Replace with the latest version
//! ```
//!
mod errors;
pub mod executer;
mod generator;
mod randomizer;
mod runner;
pub mod step;

pub use errors::{Error, Result};
pub use generator::StringDef;
pub use randomizer::Randomizer;
pub use runner::{new, Runner};
