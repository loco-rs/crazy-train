//! This module defines the [`Runner`] struct, which is responsible for managing and executing a sequence of steps in a defined order.
//! Each step can be customized with initial commands, checks, and tests, providing a flexible and extensible execution flow.
//!
//! The `Runner` can also generate and display an execution plan for the steps to be taken.
//! The steps can be randomized using the [`Randomizer`], enhancing the unpredictability of the execution.
//!
use crate::{
    executer,
    randomizer::Randomizer,
    step::{self, StepTrait},
    Error, Result,
};
use colored::Colorize;
use std::time::Instant;

/// A struct that orchestrates the execution of a series of steps.
pub struct Runner {
    steps: Vec<Box<dyn StepTrait>>,
    init: Option<Box<dyn StepTrait>>,
    randomizer: Randomizer,
}

/// Creates a new [`Runner`] instance with the given steps.
#[must_use]
pub fn new(steps: Vec<Box<dyn StepTrait>>) -> Runner {
    Runner {
        steps,
        init: None,
        randomizer: Randomizer::default(),
    }
}

impl Runner {
    /// Sets an initial step for the runner.
    #[must_use]
    pub fn init_step(mut self, step: Box<dyn StepTrait>) -> Self {
        self.init = Some(step);
        self
    }

    /// Sets a custom randomizer for the runner.
    #[must_use]
    pub fn randomizer(mut self, randomizer: Randomizer) -> Self {
        self.randomizer = randomizer;
        self
    }

    // Dumps the execution plan for the steps to be executed.
    ///
    /// # Errors
    ///
    /// when could not present the plan
    pub fn dump_plan(&self) -> Result<String> {
        let mut output: Vec<String> = Vec::new();

        output.push("====================================".to_string());
        output.push("          Execution Plan Dump        ".green().to_string());
        output.push("====================================".to_string());
        output.push(format!("{}: {}", "Step Count".bold(), &self.steps.len()));
        output.push(format!("{}: {}", "Seed".bold(), &self.randomizer.seed));
        output.push("------------------------------------".to_string());

        for (i, step) in self.steps.iter().enumerate() {
            let execution_plan = step.plan(&self.randomizer)?;
            output.push(
                format!("Step {}: {}", i + 1, execution_plan.id)
                    .green()
                    .to_string(),
            );
            output.push("------------------------------------".to_string());
            output.push("Command:".bold().to_string());
            output.push(execution_plan.command.clone());
            output.push("State:".bold().to_string());
            output.push("---".to_string());

            let state = serde_yaml::to_string(&step.to_yaml()).unwrap_or_default();
            output.push(state);

            output.push("------------------------------------".to_string());
        }

        Ok(output.join("\n"))
    }

    /// Executes the steps in the runner.
    ///
    /// # Errors
    /// On the first step that fails
    pub fn run(&self) -> Result<()> {
        println!("{}", self.dump_plan()?);
        for step in &self.steps {
            let step_plan = step.plan(&self.randomizer)?;

            println!();
            println!("{}", format!("Run step: {}", step_plan.id).yellow());
            println!();

            step.setup()?;
            let start = Instant::now();
            println!("{}", "Execute plan...".yellow());
            let result = step.plan(&self.randomizer)?.execute()?;
            println!(
                "{}",
                format!("Execute plan finished in {:?}", start.elapsed()).yellow()
            );
            let is_success =
                step.is_success(&result, &step_plan.ctx)
                    .map_err(|err| Error::StepError {
                        kind: step::Kind::Plan,
                        description: err.to_string(),
                        command_output: result,
                    })?;

            if !is_success {
                continue;
            }

            if let Some(check_command) = step.run_check() {
                let start = Instant::now();
                println!("{}", "Execute check...".yellow());
                let output = executer::run_sh(&check_command)?;
                println!(
                    "{}",
                    format!("Execute check finished in {:?}", start.elapsed()).yellow()
                );
                if output.status_code != Some(0) {
                    return Err(Error::StepError {
                        kind: step::Kind::Check,
                        description: "check not finish with status code 0".to_string(),
                        command_output: output,
                    });
                }
            }

            if let Some(test_command) = step.run_test() {
                let start = Instant::now();
                println!("{}", "Execute test...".yellow());
                let output = executer::run_sh(&test_command)?;
                println!(
                    "{}",
                    format!("Execute tests finished in {:?}", start.elapsed()).yellow()
                );
                if output.status_code != Some(0) {
                    return Err(Error::StepError {
                        kind: step::Kind::Test,
                        description: "test command not finish with status code 0".to_string(),
                        command_output: output,
                    });
                }
            }
        }

        println!("{}", "Execution plan is pass successfully".green());
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::{collections::HashMap, path::PathBuf};

    use serde::{Deserialize, Serialize};
    use step::PlanCtx;

    use super::*;
    use crate::{executer::Output, generator::StringDef, step::Plan};

    #[derive(Serialize, Deserialize)]
    struct TestStepOne {
        location: PathBuf,
    }

    #[derive(Serialize, Deserialize)]
    struct TestStepTwo {
        location: PathBuf,
    }

    impl StepTrait for TestStepOne {
        fn setup(&self) -> crate::errors::Result<()> {
            Ok(std::fs::create_dir_all(&self.location)?)
        }

        fn plan(&self, randomizer: &Randomizer) -> Result<Plan> {
            let eco_string = randomizer.string(StringDef::default()).to_string();
            Ok(Plan::with_vars::<Self>(
                format!(
                    "echo {eco_string} >> {}",
                    self.location.join("test.txt").display()
                ),
                HashMap::from([("foo".to_string(), "bar".to_string())]),
            ))
        }

        fn is_success(
            &self,
            execution_result: &Output,
            plan_ctx: &PlanCtx,
        ) -> Result<bool, &'static str> {
            if let Some(foo_var) = plan_ctx.vars.get("foo") {
                if foo_var != "bar" {
                    return Err("foo value should be equal to var");
                }
            } else {
                return Err("foo plan ctx var not found");
            };

            if execution_result.status_code == Some(0) {
                Ok(true)
            } else {
                Err("status code should be 0")
            }
        }

        fn run_check(&self) -> Option<String> {
            Some(format!(
                "test -f {}",
                self.location.join("test.txt").display()
            ))
        }

        fn run_test(&self) -> Option<String> {
            Some(format!(
                "test -f {}",
                self.location.join("test.txt").display()
            ))
        }

        fn to_yaml(&self) -> serde_yaml::Value {
            serde_yaml::to_value(self).expect("serialize")
        }
    }

    impl StepTrait for TestStepTwo {
        fn setup(&self) -> crate::errors::Result<()> {
            Ok(std::fs::create_dir_all(&self.location)?)
        }

        fn plan(&self, randomizer: &Randomizer) -> Result<Plan> {
            let eco_string = randomizer.string(StringDef::default()).to_string();
            Ok(Plan::with_vars::<Self>(
                format!(
                    "cat {eco_string} >> {}",
                    self.location.join("test.txt").display()
                ),
                HashMap::from([("foo".to_string(), "bar".to_string())]),
            ))
        }

        fn is_success(
            &self,
            execution_result: &Output,
            _plan_ctx: &PlanCtx,
        ) -> Result<bool, &'static str> {
            if execution_result.status_code == Some(1) {
                Ok(true)
            } else {
                Err("status code should be 1")
            }
        }

        fn to_yaml(&self) -> serde_yaml::Value {
            serde_yaml::to_value(self).expect("serialize")
        }
    }

    #[test]
    fn can_run() {
        let base_location = std::env::temp_dir().join("crazy-train");
        let location_step_1 = base_location.join("step-1");
        let location_step_2 = base_location.join("step-2");

        let step_one = TestStepOne {
            location: location_step_1,
        };
        let step_two = TestStepTwo {
            location: location_step_2,
        };
        let randomaizer = Randomizer::with_seed(42);
        let runner = new(vec![Box::new(step_one), Box::new(step_two)]).randomizer(randomaizer);

        assert!(runner.run().is_ok());
    }
}
