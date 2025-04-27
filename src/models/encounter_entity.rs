use hashbrown::{HashMap, HashSet};
use log::info;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DefaultOnError;
use std::fmt::{self, Display, Formatter};

use crate::models::IncapacitationEventType;

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

impl Display for EncounterEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}] (ID: {}, Class: {}, Gear: {:.1}, Type: {:?})",
            self.name,
            if self.is_dead { "Dead" } else { "Alive" },
            self.id,
            self.class_id,
            self.gear_score,
            self.entity_type
        )
    }
}

impl From<&Entity> for EncounterEntity {
    fn from(entity: &Entity) -> Self {
        let mut encounter_entity = EncounterEntity {
            id: entity.id,
            name: entity.name.clone(),
            entity_type: entity.entity_type,
            npc_id: entity.npc_id,
            class_id: entity.class_id as u32,
            class: entity.class_id.as_ref().to_string(),
            gear_score: entity.gear_level,
            ..Default::default()
        };

        if entity.character_id > 0 {
            encounter_entity.character_id = entity.character_id;
        }

        encounter_entity
    }
}

impl From<Entity> for EncounterEntity {
    fn from(entity: Entity) -> Self {
        EncounterEntity::from(&entity)
    }
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
        self.class_id = new.class_id as u32;
        self.gear_score = new.gear_level;
    }

    pub fn cap_incapacitation_durations_to_death_time(&mut self) {
        self
            .damage_stats
            .incapacitations
            .iter_mut()
            .rev()
            .take_while(|x| x.timestamp + x.duration > self.damage_stats.death_time)
            .for_each(|x| {
                // cap duration to death time if it exceeds it
                x.duration = x.timestamp - self.damage_stats.death_time;
            });
    }

    pub fn update_incapacitation(
        &mut self,
        down_time: f32,
        stand_up_time: Option<f32>,
        move_time: Option<f32>,
        timestamp: i64,
    ) {
       
        let total_incapacitated_time = down_time
            + move_time.unwrap_or_default()
            + stand_up_time.unwrap_or_default();
        let incapacitated_time_ms = (total_incapacitated_time * 1000.0) as i64;

        let prev_incapacitation = self
            .damage_stats
            .incapacitations
            .iter_mut()
            .rev()
            .take_while(|e| e.timestamp + e.duration > timestamp)
            .find(|x| x.event_type == IncapacitationEventType::FallDown);

        if let Some(prev_incapacitation) = prev_incapacitation {
            info!(
                "Shortening down duration from {} to {} because of new abnormal move",
                prev_incapacitation.duration,
                timestamp - prev_incapacitation.timestamp
            );
            prev_incapacitation.duration = timestamp - prev_incapacitation.timestamp;
        }

        let new_event = IncapacitatedEvent {
            timestamp,
            duration: incapacitated_time_ms,
            event_type: IncapacitationEventType::FallDown,
        };
        self.damage_stats.incapacitations.push(new_event);

        info!(
            "Player {} will be incapacitated for {}ms",
            self.name, incapacitated_time_ms
        );
    }

    pub fn shorten_incapacitation(&mut self, timestamp: i64) {
        let events = self
            .damage_stats
            .incapacitations
            .iter_mut()
            .rev()
            .take_while(|x| x.timestamp + x.duration > timestamp)
            .filter(|x| x.event_type == IncapacitationEventType::FallDown);

        for ongoing_event in events
        {
            info!(
                "Shortening down duration from {} to {} because of getup skill",
                ongoing_event.duration,
                timestamp - ongoing_event.timestamp
            );
            ongoing_event.duration = timestamp - ongoing_event.timestamp;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return_true_valid_npc() {
        let mut entity = EncounterEntity::default();
        entity.entity_type = EntityType::Boss;
        entity.damage_stats.damage_dealt = 1;

        assert!(entity.is_valid());
    }
}