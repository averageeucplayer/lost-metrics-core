use hashbrown::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DefaultOnError;

use super::entity::EntityType;
use super::player::ArkPassiveData;
use super::skill::Skill;
use super::DamageStats;
use super::Entity;
use super::SkillStats;
use super::{encounter::EncounterMisc, misc::IncapacitatedEvent, status_effect::StatusEffect};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncounterEntity {
    pub id: u64,
    pub character_id: u64,
    pub npc_id: u32,
    pub name: String,
    pub entity_type: EntityType,
    pub class_id: u32,
    pub class: String,
    pub gear_score: f32,
    pub current_hp: i64,
    pub max_hp: i64,
    pub current_shield: u64,
    pub is_dead: bool,
    pub skills: HashMap<u32, Skill>,
    pub damage_stats: DamageStats,
    pub skill_stats: SkillStats,
    pub engraving_data: Option<Vec<String>>,
    pub gear_hash: Option<String>,
    pub ark_passive_active: Option<bool>,
    pub ark_passive_data: Option<ArkPassiveData>,
    pub spec: Option<String>,
}

impl EncounterEntity {
    pub fn is_valid(&self) -> bool {
        ((self.entity_type == EntityType::Player && self.class_id > 0)
        || self.entity_type == EntityType::Esther
        || self.entity_type == EntityType::Boss)
        && self.damage_stats.damage_dealt > 0
    }

    pub fn is_valid_player(&self) -> bool {
        self.gear_score >= 0.0
            && self.entity_type == EntityType::Player
            && self.character_id != 0
            && self.class_id != 0
            && self.name != "You"
            && self
                .name
                .chars()
                .next()
                .unwrap_or_default()
                .is_uppercase()
    }

    pub fn is_active_player (&self, local_player: &str) -> bool {
        ((self.entity_type == EntityType::Player && self.class_id != 0 && self.max_hp > 0)
            || self.name == local_player)
            && self.damage_stats.damage_dealt > 0
    }

    pub fn is_relevant_combat_entity (&self, local_player: &str) -> bool {
        (self.entity_type == EntityType::Player && self.class_id != 0 && self.max_hp > 0)
            || self.name == local_player
            || self.entity_type == EntityType::Esther
            || (self.entity_type == EntityType::Boss && self.max_hp > 0)
            && self.damage_stats.damage_dealt > 0
    }

    pub fn update(&mut self, new: &Entity) {
        self.id = new.id;
        self.character_id = new.character_id;
        self.name.clone_from(&new.name);
        self.class_id = new.class_id;
        self.gear_score = new.gear_level;
    }
}
