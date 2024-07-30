use crate::file_system_interaction::asset_loading::{AnimationAssets, SceneAssets};
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::level_instantiation::spawning::GameObject;
use crate::movement::general_movement::{CharacterAnimations, CharacterControllerBundle, Model};
use crate::player_control::actions::{
    create_player_action_input_manager_bundle, create_ui_action_input_manager_bundle,
};
use crate::player_control::player_embodiment::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

use super::SpawnStruct;

pub(crate) const HEIGHT: f32 = 0.4;
pub(crate) const RADIUS: f32 = 0.3;

pub(crate) fn spawn(
    In(SpawnStruct { transform, ..}): In<SpawnStruct>,
    mut commands: Commands,
    animations: Res<AnimationAssets>,
    scene_handles: Res<SceneAssets>,
) {
    let entity = commands
        .spawn((
            PbrBundle {
                transform,
                ..default()
            },
            Player,
            Name::new("Player"),
            Ccd::enabled(),
            // CharacterControllerBundle::capsule(HEIGHT, RADIUS),
            CharacterControllerBundle::capsule(HEIGHT, RADIUS),
            KinematicCharacterController {
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Relative(7.0),
                    min_width: CharacterLength::Relative(0.000000000000001),
                    include_dynamic_bodies: true,
                }),
                snap_to_ground:Some(CharacterLength::Absolute(2000000000000000.0)),
                ..Default::default()
            },
            CharacterAnimations {
                idle: animations.player_character_idle.clone(),
                walk: animations.player_character_walking.clone(),
                running: animations.player_character_running.clone(),
                aerial: animations.player_character_aerial.clone(),
            },
            CollisionGroups::new(
                GameCollisionGroup::PLAYER.into(),
                GameCollisionGroup::ALL.into(),
            ),
            create_player_action_input_manager_bundle(),
            create_ui_action_input_manager_bundle(),
            GameObject::Player,
        ))
        .insert(KinematicCharacterController {
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Relative(7.0),
                min_width: CharacterLength::Relative(0.000000000000001),
                include_dynamic_bodies: true,
            }),
            snap_to_ground: Some(CharacterLength::Absolute(2000000000000000.0)),
            ..Default::default()
        },)
        .id();

    commands
        .spawn((
            Model { target: entity },
            SpatialBundle::default(),
            Name::new("Player Model Parent"),
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: scene_handles.character.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -HEIGHT / 2. - RADIUS, 0.),
                        rotation: Quat::from_rotation_y(TAU / 2.),
                        scale: Vec3::splat(0.01),
                    },
                    ..default()
                },
                KinematicCharacterController {
                autostep: Some(CharacterAutostep {
                    max_height: CharacterLength::Relative(7.0),
                    min_width: CharacterLength::Relative(0.000000000000001),
                    include_dynamic_bodies: true,
                }),
                snap_to_ground: Some(CharacterLength::Absolute(2000000000000000.0)),
                ..Default::default()
            },
                Name::new("Player Model"),
            ));
        });
}
