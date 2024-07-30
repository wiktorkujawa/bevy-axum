use crate::file_system_interaction::asset_loading::SceneAssets;
use crate::level_instantiation::spawning::GameObject;
use bevy::prelude::*;

use super::SpawnStruct;

pub(crate) fn spawn(
    In(SpawnStruct { transform, ..}): In<SpawnStruct>,
    mut commands: Commands,
    scene_handles: Res<SceneAssets>,
) {
    commands.spawn((
        SceneBundle {
            scene: scene_handles.level.clone(),
            transform,
            ..default()
        },
        Name::new("Level"),
        Imported,
        GameObject::Level,
    ));
}

#[derive(Component)]
pub(crate) struct Imported;
