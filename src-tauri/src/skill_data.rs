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

use core::fmt;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    pub id: i64,
    pub skill_type: String,
    pub skill_tier: i64,
    #[serde(rename = "requireIDList")]
    pub require_id_list: Vec<Option<serde_json::Value>>,
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

impl fmt::Display for SkillData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SkillData {{\n\
            gaksung_level: {},\n\
            attribute_type: {:?},\n\
            atk_type: {:?},\n\
            def_type: {:?},\n\
            skill_target_type: {:?},\n\
            target_num: {:?},\n\
            mp_usage: {:?},\n\
            skill_level_correction: {:?},\n\
            default_value: {:?},\n\
            can_team_kill: {:?},\n\
            can_duel: {:?},\n\
            can_change_target: {:?},\n\
            skill_motion: {:?},\n\
            view_type: {:?},\n\
            parrying_close_type: {:?},\n\
            }}",
            self.gaksung_level,
            self.attribute_type,
            self.atk_type,
            self.def_type,
            self.skill_target_type,
            self.target_num,
            self.mp_usage,
            self.skill_level_correction,
            self.default_value,
            self.can_team_kill,
            self.can_duel,
            self.can_change_target,
            self.skill_motion,
            self.view_type,
            self.parrying_close_type
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinList {
    pub operator_type: String,
    pub scale: i64,
    pub ability_script_list: Vec<AbilityScriptList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AbilityScriptList {
    pub script_name: String,
    pub buff_data: BuffData,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuffData {
    pub buff_keyword: String,
    pub target: String,
    pub buff_owner: String,
    pub stack: i64,
    pub turn: i64,
    pub active_round: i64,
}
