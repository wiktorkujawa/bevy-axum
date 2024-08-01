use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bitflags::bitflags;
use serde::{Deserialize, Serialize};

use crate::world_interaction::dialog::Dialog;

use super::GameObject;

pub(crate) mod camera;
pub(crate) mod dialog;
pub(crate) mod level;
pub(crate) mod npc;
pub(crate) mod object;
pub(crate) mod orb;
pub(crate) mod player;
pub(crate) mod point_light;
pub(crate) mod primitives;
pub(crate) mod skydome;
pub(crate) mod sunlight;
mod util;

bitflags! {
    pub(crate) struct GameCollisionGroup: u32 {
        const PLAYER = 1 << 0;
        const OTHER = 1 << 31;

        const ALL = u32::MAX;
        const NONE = 0;
    }
}

impl From<GameCollisionGroup> for Group {
    fn from(value: GameCollisionGroup) -> Self {
        // Both are u32, so this will never panic.
        Self::from_bits(value.bits()).expect("Failed to convert GameCollisionGroup to rapier Group")
    }
}

#[derive(
    Debug, Clone, Reflect, Component, PartialEq, Resource, Serialize, Deserialize, Default,
)]
pub struct SpawnStruct {
    pub transform: Transform,
    #[serde(default = "default_npc")]
    pub npc: NPCStruct,
    #[serde(default = "default_object")]
    pub object: ObjectStruct,
    #[serde(default = "default_player")]
    pub player: PlayerStruct,
}

#[derive(
    Debug, Clone, Reflect, Component, PartialEq, Resource, Serialize, Deserialize, Default,
)]
pub struct NPCStruct {
    #[serde(default = "default_scene")]
    pub scene: String,
    #[serde(default = "default_animations")]
    pub animations: Animations,
    #[serde(default = "default_dialog")]
    pub dialog: Dialog,
}

#[derive(
    Debug, Clone, Reflect, Component, PartialEq, Resource, Serialize, Deserialize, Default,
)]
pub struct ObjectStruct {
    #[serde(default = "default_scene")]
    pub scene: String,
    #[serde(default = "default_dialog")]
    pub dialog: Dialog,
}

fn default_dialog() -> Dialog {
    Dialog::default()
}

fn default_npc() -> NPCStruct {
    NPCStruct {
        scene: default_scene(),
        animations: default_animations(),
        dialog: default_dialog(),
    }
}

fn default_object() -> ObjectStruct {
    ObjectStruct {
        scene: default_scene(),
        dialog: default_dialog(),
    }
}

#[derive(
    Debug, Clone, Reflect, Component, PartialEq, Resource, Serialize, Deserialize, Default,
)]
pub struct PlayerStruct {
    #[serde(default = "default_scene")]
    pub scene: String,
    #[serde(default = "default_animations")]
    pub animations: Animations,
}

fn default_player() -> PlayerStruct {
    PlayerStruct {
        scene: default_scene(),
        animations: default_animations(),
    }
}

fn default_scene() -> String {
    String::from("scenes/Fox.glb#Scene0")
}

fn default_animations() -> Animations {
    Animations {
        character_idle: String::from("scenes/Fox.glb#Animation0"),
        character_walking: String::from("scenes/Fox.glb#Animation1"),
        character_running: String::from("scenes/Fox.glb#Animation2"),
        character_falling: String::from("scenes/Fox.glb#Animation2"),
    }
}

#[derive(
    Debug, Clone, Reflect, Component, PartialEq, Resource, Serialize, Deserialize, Default,
)]
pub struct Animations {
    pub character_idle: String,
    pub character_walking: String,
    pub character_running: String,
    pub character_falling: String,
}
