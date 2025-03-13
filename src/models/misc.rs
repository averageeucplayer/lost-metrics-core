use std::mem::transmute;

use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct DamageData {
    pub skill_id: u32,
    pub skill_effect_id: u32,
    pub damage: i64,
    pub modifier: i32,
    pub target_current_hp: i64,
    pub target_max_hp: i64,
    pub damage_attribute: Option<u8>,
    pub damage_type: u8,
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Identity {
    pub gauge1: u32,
    pub gauge2: u32,
    pub gauge3: u32,
}

#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct Stagger {
    pub current: u32,
    pub max: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct IncapacitatedEvent {
    #[serde(rename = "type")]
    pub event_type: IncapacitationEventType,
    pub timestamp: i64,

    /// Duration in milliseconds. This may be updated retroactively in a live session
    /// if the user recovers (gets up) or becomes incapacitated again with the same type of event.
    pub duration: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IncapacitationEventType {
    FallDown,
    CrowdControl,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LocalInfo {
    pub client_id: Uuid,
    pub local_players: HashMap<u64, LocalPlayer>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct LocalPlayer {
    pub name: String,
    pub count: i32,
}

#[derive(Debug, PartialEq)]
#[repr(i32)]
pub enum HitOption {
    None = 0,
    BackAttack = 1,
    FrontalAttack = 2,
    FlankAttack = 3,
    Max = 4,
}

impl From<i32> for HitOption {
    fn from(value: i32) -> Self {
        unsafe { transmute(value) }
    }
}

#[derive(Debug, PartialEq)]
#[repr(u32)]
pub enum HitFlag {
    Normal = 0,
    Critical = 1,
    Miss = 2,
    Invincible = 3,
    DamageOverTime = 4,
    Immune = 5,
    ImmuneSilenced = 6,
    FontSilence = 7,
    DamageOverTimeCritical = 8,
    Dodge = 9,
    Reflect = 10,
    DamageShare = 11,
    DodgeHit = 12,
    Max = 13,
}

impl From<i32> for HitFlag {
    fn from(value: i32) -> Self {
        unsafe { transmute(value) }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase", default)]
pub struct SkillFeatureOption {
    #[serde(rename(deserialize = "type"))]
    pub effect_type: String,
    pub level: u16,
    #[serde(rename(deserialize = "paramtype"))]
    pub param_type: String,
    pub param: Vec<i32>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PassiveOption {
    #[serde(rename(deserialize = "type"))]
    pub option_type: String,
    pub key_stat: String,
    pub key_index: i32,
    pub value: i32,
}
