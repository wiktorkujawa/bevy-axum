use crate::file_system_interaction::asset_loading::{AnimationAssets, SceneAssets};
use crate::level_instantiation::spawning::objects::{Dialog, GameCollisionGroup};
use crate::level_instantiation::spawning::GameObject;
use crate::movement::general_movement::{CharacterAnimations, CharacterControllerBundle, Model};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub(crate) const HEIGHT: f32 = 0.4;
pub(crate) const RADIUS: f32 = 0.4;

use super::SpawnStruct;

pub(crate) fn spawn(
    // In(transform): In<Transform>,
    In(SpawnStruct { transform, npc, .. }): In<SpawnStruct>,
    mut commands: Commands,
    animations_handles: Res<AnimationAssets>,
    scene_handles: Res<SceneAssets>,
) {
    let character_idle = animations_handles
        .character_idle
        .get(&npc.animations.character_idle)
        .unwrap();
    let character_walking = animations_handles
        .character_walking
        .get(&npc.animations.character_walking)
        .unwrap();
    let character_running = animations_handles
        .character_running
        .get(&npc.animations.character_running)
        .unwrap();
    let character_falling = animations_handles
        .character_falling
        .get(&npc.animations.character_falling)
        .unwrap();

    let entity = commands
        .spawn((
            PbrBundle {
                transform,
                ..default()
            },
            npc.clone(),
            Name::new("NPC"),
            CharacterControllerBundle::capsule(HEIGHT, RADIUS),
            CharacterAnimations {
                idle: character_idle.clone(),
                walk: character_walking.clone(),
                running: character_running.clone(),
                aerial: character_falling.clone(),
            },
            Dialog { ..npc.dialog },
            GameObject::Npc,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("NPC Dialog Collider"),
                Collider::cylinder(HEIGHT / 2., RADIUS * 5.),
                Sensor,
                ActiveEvents::COLLISION_EVENTS,
                ActiveCollisionTypes::DYNAMIC_DYNAMIC,
                CollisionGroups::new(
                    GameCollisionGroup::OTHER.into(),
                    GameCollisionGroup::PLAYER.into(),
                ),
            ));
        })
        .id();

    let scene = scene_handles.npc.get(&npc.scene).unwrap();

    commands
        .spawn((
            Model { target: entity },
            SpatialBundle::default(),
            Name::new("NPC Model Parent"),
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: scene.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -HEIGHT / 2. - RADIUS, 0.),
                        scale: Vec3::splat(0.012),
                        rotation: Quat::from_rotation_y(TAU / 2.),
                    },
                    ..default()
                },
                Name::new("NPC Model"),
            ));
        });
}
