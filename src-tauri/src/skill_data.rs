// Example code that deserializes and serializes the model.
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;
//
// use generated_module::Skill;
//
// fn main() {
//     let json = r#"{"answer": 42}"#;
//     let model: Skill = serde_json::from_str(&json).unwrap();
// }

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub id: Option<i64>,
    pub skill_type: Option<String>,
    pub skill_tier: Option<i64>,
    #[serde(rename = "requireIDList")]
    pub require_id_list: Option<Vec<Option<String>>>,
    pub skill_data: Vec<SkillData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkillData {
    pub gaksung_level: i64,
    pub attribute_type: Option<String>,
    pub atk_type: Option<String>,
    pub def_type: Option<String>,
    pub skill_target_type: Option<String>,
    pub target_num: Option<i64>,
    pub mp_usage: Option<i64>,
    pub skill_level_correction: Option<i64>,
    pub default_value: Option<i64>,
    pub can_team_kill: Option<bool>,
    pub can_duel: Option<bool>,
    pub can_change_target: Option<bool>,
    pub skill_motion: Option<String>,
    pub view_type: Option<String>,
    pub parrying_close_type: Option<String>,
    pub coin_list: Option<Vec<CoinList>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinList {
    pub operator_type: String,
    pub scale: i64,
    pub ability_script_list: Option<Vec<AbilityScriptList>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityScriptList {
    pub script_name: String,
    pub buff_data: Option<BuffData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuffData {
    pub buff_keyword: Option<String>,
    pub target: Option<String>,
    pub buff_owner: Option<String>,
    pub stack: Option<i64>,
    pub turn: Option<i64>,
    pub active_round: Option<i64>,
}
