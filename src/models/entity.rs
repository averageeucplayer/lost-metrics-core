use std::fmt::Display;
use std::str::FromStr;
use hashbrown::HashMap;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

#[derive(Debug, Default, Clone)]
pub struct Entity {
    pub id: u64,
    pub entity_type: EntityType,
    pub name: String,
    pub npc_id: u32,
    pub class_id: u32,
    pub gear_level: f32,
    pub character_id: u64,
    pub owner_id: u64,
    pub skill_effect_id: u32,
    pub skill_id: u32,
    pub stats: HashMap<u8, i64>,
    pub stance: u8,
    pub grade: String,
    pub push_immune: bool,
    pub level: u16,
    pub balance_level: u16,
}

#[derive(Display, Debug, EnumString, Default, Serialize, Deserialize, PartialEq, Copy, Clone)]
#[strum(serialize_all = "PascalCase")]
pub enum EntityType {
    #[default]
    Unknown,
    Monster,
    Boss,
    Guardian,
    Player,
    Npc,
    Esther,
    Projectile,
    Summon,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Npc {
    pub id: i32,
    pub name: Option<String>,
    pub grade: NpcGrade,
    #[serde(rename = "type")]
    pub npc_type: String,
}

#[derive(Debug, Display, Default, Serialize, Deserialize, PartialEq, Eq, Hash, Copy, Clone)]
#[serde(rename_all = "lowercase")]
pub enum NpcGrade {
    #[default]
    None,
    Normal,
    Boss,
    Elite,
    Commander,
    Lucky,
    Raid,
    #[serde(rename = "epic_raid")]
    EpicRaid,
    Named,
    Underling,
    Seed
}

impl Npc {
    pub fn is_boss(&self) -> bool {
        match self.grade {
            NpcGrade::Boss 
            | NpcGrade::Commander
            | NpcGrade::Raid
            | NpcGrade::EpicRaid => true,
            _ => false
        }
    }

    pub fn has_valid_name(&self) -> bool {

        if let Some(name) = &self.name {
            let contains_underscore = name.contains('_');
            let all_ascii = name.chars().all(|c| c.is_ascii());

            return contains_underscore && all_ascii;
        }
        
        false
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct Esther {
    pub name: String,
    pub icon: String,
    pub skills: Vec<i32>,
    #[serde(alias = "npcs")]
    pub npc_ids: Vec<u32>,
}
