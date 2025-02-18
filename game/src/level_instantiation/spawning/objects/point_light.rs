use crate::level_instantiation::spawning::GameObject;

use bevy::prelude::*;

use super::SpawnStruct;

pub(crate) fn spawn(In(SpawnStruct { transform, .. }): In<SpawnStruct>, mut commands: Commands) {
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                color: Color::WHITE,
                intensity: 1.0,
                range: 1.0,
                radius: 1.0,
                shadows_enabled: true,
                ..default()
            },
            transform,
            ..default()
        },
        Name::new("Light"),
        GameObject::PointLight,
    ));
}
