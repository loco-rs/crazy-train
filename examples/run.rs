use crazy_train::{
    step::{Plan, PlanCtx, StepTrait},
    Result, StringDef,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct StepOne {}

impl StepTrait for StepOne {
    fn plan(&self, randomizer: &crazy_train::Randomizer) -> Result<crazy_train::step::Plan> {
        let eco_string = randomizer.string(StringDef::default()).to_string();

        Ok(Plan::new::<Self>(format!("echo {eco_string}")))
    }

    fn is_success(
        &self,
        execution_result: &crazy_train::executer::Output,
        _plan_ctx: &PlanCtx,
    ) -> Result<bool, &'static str> {
        if execution_result.status_code == Some(0) {
            Ok(true)
        } else {
            Err("status code should be 0")
        }
    }

    fn to_yaml(&self) -> serde_yaml::Value {
        serde_yaml::to_value(self).expect("serialize")
    }
}

#[derive(Serialize, Deserialize)]
struct StepTwo {}

impl StepTrait for StepTwo {
    fn plan(&self, randomizer: &crazy_train::Randomizer) -> Result<crazy_train::step::Plan> {
        let eco_string = randomizer.string(StringDef::default()).to_string();

        Ok(Plan::new::<Self>(format!("unknown-command {eco_string}")))
    }

    fn is_success(
        &self,
        execution_result: &crazy_train::executer::Output,
        _plan_ctx: &PlanCtx,
    ) -> Result<bool, &'static str> {
        if execution_result.status_code == Some(0) {
            Err("expected failure command")
        } else {
            Ok(true)
        }
    }

    fn to_yaml(&self) -> serde_yaml::Value {
        serde_yaml::to_value(self).expect("serialize")
    }
}

fn main() {
    let step_one = StepOne {};
    let step_two = StepTwo {};
    let runner = crazy_train::new(vec![Box::new(step_one), Box::new(step_two)]);

    let res = runner.run();
    println!("{res:#?}");
}
