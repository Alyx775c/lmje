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
    pub attribute_type: Option<AttributeType>,
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

#[derive(Serialize, Clone, Deserialize, Debug)]
pub enum AttributeType {
    NEUTRAL,
    CRIMSON,
    SCARLET,
    AMBER,
    SHAMROCK,
    AZURE,
    INDIGO,
    VIOLET,
    WHITE,
    BLACK,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinList {
    pub operator_type: CoinTypes,
    pub scale: i64,
    pub ability_script_list: Option<Vec<AbilityScriptList>>,
}

#[derive(Serialize, Clone, Deserialize, Debug)]
pub enum CoinTypes {
    ADD,
    MUL,
    SUB,
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

/*
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skill {
    id: i32,
    text_id: Option<i32>,
    skill_tier: i32,
    skill_data: Vec<SkillData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SkillData {
    action_script: Option<String>,
    icon_id: Option<String>,
    gaksung_level: Option<i32>,
    can_team_kill: Option<bool>,
    can_duel: Option<bool>,
    can_change_target: Option<bool>,
    attribute_type: Option<AttributeType>,
    atk_type: Option<AttackType>,
    def_type: Option<DefinitionType>,
    skill_motion: Option<SkillMotion>,
    view_type: Option<ViewType>,
    #[serde(rename = "arearange")]
    area_range: Option<i32>,
    parrying_close_type: Option<String>,
    #[serde(rename = "targetNum")]
    atk_weight: Option<i32>,       
    #[serde(rename = "defaultValue")]
    starting_value: Option<i32>,    
    #[serde(rename = "skillLevelCorrection")]
    offense_level_incr: Option<i32>,
    skill_target_type: Option<TargetType>,
    ability_script_list: Option<Vec<AbilityScript>>,
    coin_list: Option<Vec<CoinData>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CoinData {
    operator_type: CoinTypes,
    #[serde(rename = "scale")]
    value: i32, // scale
    ability_script_list: Option<Vec<AbilityScript>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CoinTypes {
    ADD,
    MUL,
    SUB,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbilityScript {
    script_name: String,
    value: Option<f32>,
    buff_data: Option<BuffData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuffData {
    buff_keyword: BuffKeyword,
    target: BuffTarget,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum BuffKeyword {}

#[derive(Serialize, Deserialize, Debug)]
pub enum BuffTarget {
    SELF,
    TARGET,
    EVERYALLY,
    EVERYENEMY,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AttributeType {
    CRIMSON,
    SCARLET,
    AMBER,
    SHAMROCK,
    AZURE,
    INDIGO,
    VIOLET,
    WHITE,
    BLACK,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AttackType {
    SLASH,
    PENETRATE,
    HIT,
    NONE,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DefinitionType {
    ATTACK,
    GUARD,
    EVADE,
    COUNTER,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SkillMotion {
    Default,
    Evade,
    S1,
    S2,
    S3,
    S4,
    S5,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ViewType {
    BATTLE,
    ENCOUNTER,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum TargetType {
    RANDOM,
    FRONT,
}
*/