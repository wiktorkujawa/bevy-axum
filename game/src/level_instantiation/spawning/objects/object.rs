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
    In(SpawnStruct { transform, object, ..}): In<SpawnStruct>,    
    mut commands: Commands,
    scene_handles: Res<SceneAssets>,
) {

    let entity = commands
        .spawn((
            PbrBundle {
                transform,
                ..default()
            },
            object.clone(),
            Name::new("Object"),
            CharacterControllerBundle::capsule(HEIGHT, RADIUS),
            Dialog { ..object.dialog },
            GameObject::Object,
        ))
        .with_children(|parent| {
            parent.spawn((
                Name::new("Object Dialog Collider"),
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

    let scene = scene_handles.object.get(&object.scene).unwrap();

    commands
        .spawn((
            Model { target: entity },
            SpatialBundle::default(),
            Name::new("Object Model Parent"),
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
                Name::new("Object Model"),
            ));
        });
}