use crate::file_system_interaction::level_serialization::{CurrentLevel, WorldLoadRequest};
use crate::level_instantiation::spawning::objects::SpawnStruct;
use crate::level_instantiation::spawning::GameObject;
use crate::player_control::player_embodiment::Player;
use crate::world_interaction::condition::ActiveConditions;
use crate::GameState;
use anyhow::{Context, Error, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::*;
use chrono::prelude::Local;
use glob::glob;
use serde::{Deserialize, Serialize};
use spew::prelude::*;
use std::borrow::Cow;
use std::fs;
use std::path::PathBuf;

pub(crate) fn game_state_serialization_plugin(app: &mut App) {
    app.add_event::<GameSaveRequest>()
        .add_event::<GameLoadRequest>()
        .add_systems(
            Update,
            (
                handle_load_requests,
                handle_save_requests.run_if(resource_exists::<CurrentLevel>()),
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Event, Serialize, Deserialize, Default)]
pub(crate) struct GameSaveRequest {
    pub(crate) filename: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Event, Serialize, Deserialize, Default)]
pub(crate) struct GameLoadRequest {
    pub(crate) filename: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Resource, Serialize, Deserialize, Default)]
struct SaveModel {
    scene: String,
    #[serde(default, skip_serializing_if = "ActiveConditions::is_empty")]
    conditions: ActiveConditions,
    player_transform: Transform,
}

#[sysfail(log(level = "error"))]
fn handle_load_requests(
    mut commands: Commands,
    mut load_events: EventReader<GameLoadRequest>,
    mut loader: EventWriter<WorldLoadRequest>,
    mut spawner: EventWriter<SpawnEvent<GameObject, SpawnStruct>>,
) -> Result<()> {
    for load in load_events.read() {
        let path = match load
            .filename
            .as_ref()
            .map(|filename| anyhow::Ok(Some(get_save_path(filename.clone()))))
            .unwrap_or_else(read_last_save)?
        {
            Some(path) => path,
            None => {
                error!("Failed to load save: No filename provided and no saves found on disk");
                continue;
            }
        };
        let serialized = match fs::read_to_string(&path) {
            Ok(serialized) => {
                info!("Successfully read save at {}", path.to_string_lossy());
                serialized
            }
            Err(e) => {
                error!(
                    "Failed to read save {:?} at {:?}: {}",
                    &load.filename, path, e
                );
                continue;
            }
        };
        let save_model: SaveModel = match ron::from_str(&serialized) {
            Ok(save_model) => save_model,
            Err(e) => {
                error!(
                    "Failed to deserialize save {:?} at {:?}: {}",
                    &load.filename, path, e
                );
                continue;
            }
        };
        loader.send(WorldLoadRequest {
            filename: save_model.scene,
        });
        commands.insert_resource(save_model.conditions);

        spawner.send(
            SpawnEvent::with_data(
                GameObject::Player,
                SpawnStruct {
                    transform: save_model.player_transform,
                    ..Default::default()
                },
            )
            .delay_frames(2),
        );

        spawner.send(
            SpawnEvent::with_data(GameObject::Dialog, SpawnStruct::default()).delay_frames(2),
        );
    }
    Ok(())
}

fn read_last_save() -> Result<Option<PathBuf>, Error> {
    let mut saves: Vec<_> = glob("./saves/*.sav.ron")
        .context("Failed to read glob pattern")?
        // .context("Failed to read glob pattern").unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.is_file())
        .collect();
    saves.sort_by_cached_key(|f| {
        f.metadata()
            .expect("Failed to read file metadata")
            .modified()
            .expect("Failed to read file modified time")
    });
    let save = saves.last().map(|entry| entry.to_owned());
    Ok(save)
}

#[sysfail(log(level = "error"))]
fn handle_save_requests(
    mut save_events: EventReader<GameSaveRequest>,
    conditions: Res<ActiveConditions>,
    player_query: Query<&GlobalTransform, With<Player>>,
    current_level: Res<CurrentLevel>,
) -> Result<()> {
    for save in save_events.read() {
        for player in &player_query {
            let save_model = SaveModel {
                scene: current_level.scene.clone(),
                conditions: conditions.clone(),
                player_transform: player.compute_transform(),
            };
            let serialized = match ron::to_string(&save_model) {
                Ok(string) => string,
                Err(e) => {
                    error!("Failed to save world: {}", e);
                    continue;
                }
            };
            let filename = save
                .filename
                .clone()
                .unwrap_or_else(|| Local::now().to_rfc2822().replace(':', "-"));
            let path = get_save_path(filename.clone());
            let dir = path.parent().context("Failed to get save directory")?;
            fs::create_dir_all(dir).context("Failed to create save directory")?;
            fs::write(&path, serialized)
                .unwrap_or_else(|e| error!("Failed to write save {filename}: {e}"));

            info!("Successfully saved game at {}", path.to_string_lossy());
        }
    }
    Ok(())
}

fn get_save_path(filename: impl Into<Cow<'static, str>>) -> PathBuf {
    let filename = filename.into().to_string();
    format!("saves/{filename}.sav.ron").into()
}
