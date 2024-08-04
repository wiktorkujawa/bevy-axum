use crate::file_system_interaction::asset_loading::LevelAssets;
use crate::level_instantiation::spawning::objects::SpawnStruct;
use crate::level_instantiation::spawning::GameObject;
use crate::world_interaction::condition::ActiveConditions;
// use crate::world_interaction::dialog::CurrentDialog;
use crate::world_interaction::interactions_ui::InteractionOpportunities;
use anyhow::{Context, Result};
use bevy::prelude::*;

use bevy_mod_sysfail::*;
use serde::{Deserialize, Serialize};
use spew::prelude::*;
use std::path::Path;
use std::{fs, iter};

pub(crate) fn level_serialization_plugin(app: &mut App) {
    app.add_event::<WorldSaveRequest>()
        .add_event::<WorldLoadRequest>()
        .add_systems(
            PostUpdate,
            (
                save_world,
                load_world.run_if(resource_exists::<LevelAssets>()),
            ),
        );
}

#[derive(Debug, Clone, Eq, PartialEq, Event, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct WorldSaveRequest {
    pub(crate) filename: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Event, Reflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct WorldLoadRequest {
    pub(crate) filename: String,
}

#[derive(Debug, Clone, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub(crate) struct CurrentLevel {
    pub(crate) scene: String,
}

#[sysfail(log(level = "error"))]
fn save_world(
    mut save_requests: EventReader<WorldSaveRequest>,
    spawn_query: Query<(&GameObject, Option<&SpawnStruct>)>,
) -> Result<()> {
    for save in save_requests.read() {
        let scene = save.filename.clone();
        let valid_candidates: Vec<_> = iter::once(scene.clone())
            .chain((1..).map(|n| format!("{0}-{n}", scene.clone())))
            .map(|filename| {
                Path::new("assets")
                    .join("levels")
                    .join(filename)
                    .with_extension("lvl.ron")
            })
            .map(|path| (path.clone(), fs::metadata(path).is_ok()))
            .take(10)
            .filter_map(|(path, exists)| Some((path, exists)))
            .collect();
        if valid_candidates.is_empty() {
            error!("Failed to save scene \"{}\": Invalid path", scene);
        } else if let Some(path) = valid_candidates
            .iter()
            .filter_map(|(path, exists)| (!exists).then_some(path))
            .next()
        {
            let serialized_world = serialize_world(&spawn_query)?;
            let dir = path.parent().context("Failed to get level directory")?;
            fs::create_dir_all(dir).context("Failed to create level directory")?;

            // let serialized_world = serialize_world(&spawn_query).unwrap();
            // let dir = path.parent().context("Failed to get level directory").unwrap();
            // fs::create_dir_all(dir).context("Failed to create level directory").unwrap();

            fs::write(path, serialized_world)
                .unwrap_or_else(|e| error!("Failed to save level \"{}\": {}", scene, e));
            info!(
                "Successfully saved level \"{}\" at {}",
                scene,
                path.to_string_lossy()
            );
        } else {
            error!(
                "Failed to save level \"{}\": Already got too many saves with this name",
                scene
            );
        }
    }
    Ok(())
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Protected;

#[sysfail(log(level = "error"))]
fn load_world(
    mut commands: Commands,
    mut load_requests: EventReader<WorldLoadRequest>,
    current_spawn_query: Query<Entity, With<GameObject>>,
    mut spawn_requests: EventWriter<SpawnEvent<GameObject, SpawnStruct>>,
    levels: Res<Assets<SerializedLevel>>,
    level_handles: Res<LevelAssets>,
) -> Result<()> {
    for load in load_requests.read() {
        let path = format!("levels/{}.lvl.ron", load.filename.clone());
        let handle = match level_handles.levels.get(&path) {
            Some(handle) => handle,
            None => {
                error!(
                    "Failed to load scene \"{}\": No such level. Available levels: {:?}",
                    path,
                    level_handles.levels.keys()
                );
                continue;
            }
        };
        let spawn_events = &levels
            .get(handle)
            .context("Failed to get level from handle in level assets")?;
        // .context("Failed to get level from handle in level assets").unwrap();
        let spawn_events = Vec::<SpawnEvent<GameObject, SpawnStruct>>::from(*spawn_events);
        for entity in &current_spawn_query {
            commands
                .get_entity(entity)
                .context("Failed to get entity while loading")?
                // .context("Failed to get entity while loading").unwrap()
                .despawn_recursive();
        }
        for event in spawn_events.into_iter() {
            spawn_requests.send(event);
        }
        commands.insert_resource(CurrentLevel {
            scene: load.filename.clone(),
        });
        commands.insert_resource(InteractionOpportunities::default());
        commands.insert_resource(ActiveConditions::default());
        // commands.remove_resource::<CurrentDialog>();

        info!("Successfully loaded scene \"{}\"", load.filename,)
    }
    Ok(())
}

fn serialize_world(spawn_query: &Query<(&GameObject, Option<&SpawnStruct>)>) -> Result<String> {
    let objects: Vec<_> = spawn_query
        .iter()
        .filter(|(game_object, _)| **game_object != GameObject::Player)
        .map(|(game_object, transform)| {
            SpawnEvent::with_data(*game_object, transform.cloned().unwrap_or_default())
        })
        .collect();
    let serialized_level = SerializedLevel::from(objects);
    ron::ser::to_string_pretty(&serialized_level, default()).context("Failed to serialize world")
}

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, Asset, Deref, DerefMut)]
#[reflect(Serialize, Deserialize)]
pub(crate) struct SerializedLevel(pub(crate) Vec<(GameObject, SpawnStruct)>);

impl From<Vec<SpawnEvent<GameObject, SpawnStruct>>> for SerializedLevel {
    fn from(events: Vec<SpawnEvent<GameObject, SpawnStruct>>) -> Self {
        Self(
            events
                .into_iter()
                .map(|event| (event.object, event.data))
                .collect(),
        )
    }
}

impl From<&SerializedLevel> for Vec<SpawnEvent<GameObject, SpawnStruct>> {
    fn from(level: &SerializedLevel) -> Self {
        level
            .iter()
            .map(|(object, transform)| SpawnEvent::with_data(*object, (*transform).clone()))
            .collect()
    }
}
