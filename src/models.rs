use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct ActionPlan {
    pub goal: String,
    pub steps: Vec<ActionStep>,
    pub difficulty: Difficulty,
    pub estimated_minutes: u32,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct ActionStep {
    pub index: u8,
    pub description: String,
    pub tool_hint: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}
 