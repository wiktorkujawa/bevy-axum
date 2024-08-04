use crate::level_instantiation::spawning::objects::util::MeshAssetsExt;
use crate::level_instantiation::spawning::GameObject;
use crate::shader::Materials;
use bevy::pbr::{NotShadowCaster, NotShadowReceiver};
use bevy::prelude::*;

use super::SpawnStruct;

fn get_or_add_mesh_handle(mesh_assets: &mut Assets<Mesh>) -> Handle<Mesh> {
    const MESH_HANDLE: Handle<Mesh> = Handle::weak_from_u128(0x1f40128bac02a9b);
    mesh_assets.get_or_add(MESH_HANDLE, || {
        Mesh::from(shape::UVSphere {
            radius: 1.0,
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
    commands
        .spawn((
            MaterialMeshBundle {
                mesh: mesh_handle,
                material: materials.glowy.clone(),
                transform,
                ..default()
            },
            Name::new("Orb"),
            NotShadowCaster,
            NotShadowReceiver,
            GameObject::Orb,
        ))
        .with_children(|parent| {
            parent.spawn((PointLightBundle {
                point_light: PointLight {
                    intensity: 10_000.,
                    radius: 1.,
                    color: Color::rgb(0.5, 0.1, 0.),
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            },));
        });
}
