use crate::level_instantiation::spawning::objects::util::MeshAssetsExt;
use crate::level_instantiation::spawning::GameObject;
use crate::shader::Materials;

use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;

use serde::{Deserialize, Serialize};

use super::SpawnStruct;

#[derive(Debug, Component, Clone, Copy, Serialize, Deserialize, Reflect, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Skydome;

fn get_or_add_mesh_handle(mesh_assets: &mut Assets<Mesh>) -> Handle<Mesh> {
    const MESH_HANDLE: Handle<Mesh> = Handle::weak_from_u128(0x1f40128bac02a9a);
    mesh_assets.get_or_add(MESH_HANDLE, || {
        Mesh::from(shape::UVSphere {
            radius: 150.0,
            ..default()
        })
    })
}

pub(crate) fn spawn(
    In(SpawnStruct { transform, .. }): In<SpawnStruct>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Materials>,
) {
    let mesh_handle = get_or_add_mesh_handle(&mut meshes);
    commands.spawn((
        Name::new("Skydome"),
        NotShadowCaster,
        NotShadowReceiver,
        Skydome,
        MaterialMeshBundle {
            mesh: mesh_handle,
            material: materials.skydome.clone(),
            transform,
            ..default()
        },
        GameObject::Skydome,
    ));
}
