//! This module provides functionality for executing shell commands and capturing their outputs.
//!
//! The [`Output`] struct represents the output of a shell command, including the status code,
//! standard output (stdout), and standard error (stderr).

use crate::errors::Result;

/// Represents the output of a shell command execution.
#[derive(Debug)]
pub struct Output {
    /// The exit status code of the command. It is optional to accommodate commands that may not
    /// return a status code.
    pub status_code: Option<i32>,
    /// The standard output produced by the command.
    pub stdout: String,
    /// The standard error output produced by the command.
    pub stderr: String,
}

/// Executes a shell command and returns its output.
///
/// # Errors
///
/// This function will return an error if:
/// - The command fails to execute.
/// - There is an error capturing the output or converting it to a UTF-8 string.
pub fn run_sh(command: &str) -> Result<Output> {
    let output = duct_sh::sh_dangerous(command)
        .stderr_capture()
        .stderr_capture()
        .unchecked()
        .run()?;

    Ok(Output {
        status_code: output.status.code(),
        stdout: std::str::from_utf8(&output.stdout)?.to_string(),
        stderr: std::str::from_utf8(&output.stderr)?.to_string(),
    })
}
