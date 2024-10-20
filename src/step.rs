//! This module defines the `StepTrait` trait, which outlines the necessary methods that
//! any step in the execution process must implement. It also defines the `Plan` struct,
//! which encapsulates a command to be executed as part of a step.
//!

use std::collections::HashMap;

use crate::{
    errors,
    executer::{self, Output},
    randomizer::Randomizer,
};

/// Enum representing the different types of steps that can be executed.
#[derive(Debug)]
pub enum Kind {
    Setup,
    Plan,
    Check,
    Test,
}

/// A trait that defines the behavior required for steps in the execution process.
#[allow(clippy::module_name_repetitions)]
pub trait StepTrait {
    /// Prepares the setup by creating necessary directories and performing initialization steps.
    ///
    /// # Errors
    ///
    /// Returns an error if the setup fails, such as when it is unable to create the required directory.
    fn setup(&self) -> errors::Result<()> {
        Ok(())
    }
    /// Generates a plan for execution.
    ///
    /// # Errors
    ///
    /// when could not prepare the plan
    fn plan(&self, randomizer: &Randomizer) -> errors::Result<Plan>;

    /// Determines if the execution result indicates success for this step.
    ///
    /// the bool result point if the runner should continue to the next steps or not.
    ///
    /// # Errors
    /// When plan result parsing is not the expected behavior.
    fn is_success(&self, execution_result: &Output) -> Result<bool, &'static str>;

    /// Optionally returns a command to run as a check after the execution of the plan.
    fn run_check(&self) -> Option<String> {
        None
    }

    /// Optionally returns a command to run as a test after the execution of the plan.
    fn run_test(&self) -> Option<String> {
        None
    }

    /// Serializes the step to a YAML representation.
    fn to_yaml(&self) -> serde_yaml::Value;
}

/// A struct that represents a plan for executing a command as part of a step.
#[derive(Debug, Clone)]
pub struct Plan {
    pub id: String,
    pub command: String,
    pub ctx: Option<HashMap<String, String>>,
}

impl Plan {
    /// Executes the command defined in the plan.
    ///
    /// # Errors
    ///
    /// on shell command failure.
    pub fn execute(&self) -> errors::Result<executer::Output> {
        executer::run_sh(&self.command)
    }

    #[must_use]
    pub fn new<T>(command: impl Into<String>) -> Self {
        Self {
            id: std::any::type_name::<T>().to_string(),
            command: command.into(),
            ctx: None,
        }
    }

    #[must_use]
    pub fn with_ctx<T>(command: impl Into<String>, ctx: HashMap<String, String>) -> Self {
        Self {
            id: std::any::type_name::<T>().to_string(),
            command: command.into(),
            ctx: Some(ctx),
        }
    }
}
