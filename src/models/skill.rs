use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use crate::models::utils::int_or_string_as_string;

use super::misc::{PassiveOption, SkillFeatureOption};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillCast {
    pub timestamp: i64,
    pub last: i64,
    pub hits: Vec<SkillHit>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillHit {
    pub timestamp: i64,
    pub damage: i64,
    pub crit: bool,
    pub back_attack: bool,
    pub front_attack: bool,
    pub buffed_by: Vec<u32>,
    pub debuffed_by: Vec<u32>,
    pub rdps_damage_received: i64,
    pub rdps_damage_received_support: i64,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillData {
    pub id: i32,
    pub name: Option<String>,
    #[serde(rename = "type", default)]
    pub skill_type: String,
    pub desc: Option<String>,
    pub class_id: u32,
    pub icon: Option<String>,
    pub identity_category: Option<String>,
    #[serde(alias = "groups")]
    pub groups: Option<Vec<i32>>,
    pub summon_source_skills: Option<Vec<u32>>,
    pub source_skills: Option<Vec<u32>>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct Skill {
    pub id: u32,
    pub name: String,
    pub icon: String,
    pub total_damage: i64,
    pub max_damage: i64,
    pub max_damage_cast: i64,
    pub buffed_by: HashMap<u32, i64>,
    pub debuffed_by: HashMap<u32, i64>,
    pub buffed_by_support: i64,
    pub buffed_by_identity: i64,
    pub buffed_by_hat: i64,
    pub debuffed_by_support: i64,
    pub casts: i64,
    pub hits: i64,
    pub crits: i64,
    pub adjusted_crit: Option<f64>,
    pub crit_damage: i64,
    pub back_attacks: i64,
    pub front_attacks: i64,
    pub back_attack_damage: i64,
    pub front_attack_damage: i64,
    pub dps: i64,
    pub cast_log: Vec<i32>,
    pub tripod_index: Option<TripodIndex>,
    pub tripod_level: Option<TripodLevel>,
    pub gem_cooldown: Option<u8>,
    pub gem_tier: Option<u8>,
    pub gem_damage: Option<u8>,
    pub gem_tier_dmg: Option<u8>,
    #[serde(skip)]
    pub tripod_data: Option<Vec<TripodData>>,
    #[serde(skip)]
    pub summon_sources: Option<Vec<u32>>,
    pub rdps_damage_received: i64,
    pub rdps_damage_received_support: i64,
    pub rdps_damage_given: i64,
    pub skill_cast_log: Vec<SkillCast>,
    #[serde(skip)]
    pub last_timestamp: i64,
}


#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct TripodData {
    pub index: u8,
    pub options: Vec<SkillFeatureOption>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase", default)]
pub struct TripodLevel {
    pub first: u16,
    pub second: u16,
    pub third: u16,
}

impl PartialEq for TripodLevel {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second && self.third == other.third
    }
}

impl Eq for TripodLevel {}

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "camelCase", default)]
pub struct TripodIndex {
    pub first: u8,
    pub second: u8,
    pub third: u8,
}

impl PartialEq for TripodIndex {
    fn eq(&self, other: &Self) -> bool {
        self.first == other.first && self.second == other.second && self.third == other.third
    }
}

impl Eq for TripodIndex {}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillEffectData {
    pub id: i32,
    pub comment: String,
    #[serde(skip)]
    pub stagger: i32,
    pub source_skills: Option<Vec<u32>>,
    pub directional_mask: Option<i32>,
    pub item_name: Option<String>,
    pub item_desc: Option<String>,
    pub item_type: Option<String>,
    pub icon: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SkillBuffData {
    pub id: i32,
    pub name: Option<String>,
    pub desc: Option<String>,
    pub icon: Option<String>,
    pub icon_show_type: Option<String>,
    pub duration: i32,
    // buff | debuff
    pub category: String,
    #[serde(rename(deserialize = "type"))]
    #[serde(deserialize_with = "int_or_string_as_string")]
    pub buff_type: String,
    pub status_effect_values: Option<Vec<i32>>,
    pub buff_category: Option<String>,
    pub target: String,
    pub unique_group: u32,
    #[serde(rename(deserialize = "overlap"))]
    pub overlap_flag: i32,
    pub passive_options: Vec<PassiveOption>,
    pub source_skills: Option<Vec<u32>>,
    pub set_name: Option<String>,
}