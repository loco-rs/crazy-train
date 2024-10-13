//! This module defines custom error types used in the Crazy Train library.
//!
//! The [`Error`] enum provides a comprehensive way to handle errors that may occur during the
//! execution of command steps and related operations.
//!
use crate::{executer::Output, step};

/// Represents errors that can occur in the Crazy Train library.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error indicating that a specific step in the execution process has failed.
    #[error("Step failed: {:?}.\ndescription: {description}.\nstatus code: {:?}.\nstdout: {}.\nstderr: {}", kind, command_output.status_code, command_output.stdout, command_output.stderr)]
    StepError {
        kind: step::Kind,
        description: String,
        command_output: Output,
    },

    /// An error indicating a failure in input/output operations.
    #[error(transparent)]
    IO(#[from] std::io::Error),

    /// An error for UTF-8 conversion failures.
    #[error("UTF-8 conversion error: {0}")]
    Utf8(#[from] std::str::Utf8Error),

    /// A generic error type that captures any string error.
    #[error("{0}")]
    Any(String),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
