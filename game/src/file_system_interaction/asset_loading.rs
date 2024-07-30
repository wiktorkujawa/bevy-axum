use crate::file_system_interaction::config::GameConfig;
use crate::file_system_interaction::level_serialization::SerializedLevel;
use crate::GameState;
use anyhow::Result;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use bevy_common_assets::toml::TomlAssetPlugin;
use bevy_egui::egui::ProgressBar;
use bevy_egui::{egui, EguiContexts};
use bevy_kira_audio::AudioSource;
use bevy_mod_sysfail::*;
use iyes_progress::{ProgressCounter, ProgressPlugin};

pub(crate) fn loading_plugin(app: &mut App) {
    app.add_plugins(RonAssetPlugin::<SerializedLevel>::new(&["lvl.ron"]))
        .add_plugins(TomlAssetPlugin::<GameConfig>::new(&["game.toml"]))
        .add_plugins(ProgressPlugin::new(GameState::Loading).continue_to(GameState::Menu))
        .add_loading_state(
            LoadingState::new(GameState::Loading)
                .continue_to_state(GameState::Menu)
                .load_collection::<AudioAssets>()
                .load_collection::<SceneAssets>()
                .load_collection::<ImageAssets>()
                .load_collection::<AnimationAssets>()
                .load_collection::<LevelAssets>()
                .load_collection::<TextureAssets>()
                .load_collection::<ConfigAssets>(),
        )
        .add_systems(Update, show_progress.run_if(in_state(GameState::Loading)))
        .add_systems(Update, update_config);
}

// the following asset collections will be loaded during the State `GameState::InitialLoading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct AudioAssets {
    #[asset(path = "audio/walking.ogg")]
    pub(crate) walking: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct SceneAssets {
    #[asset(path = "scenes/main_character.glb#Scene0")]
    pub(crate) character: Handle<Scene>,
    #[asset(paths(
        "scenes/Fox.glb#Scene0", 
        "scenes/main_character.glb#Scene0", 
        "scenes/electrician.glb#Scene0",
        "scenes/scientist.glb#Scene0"), collection(typed, mapped))]
    pub(crate) npc: HashMap<String, Handle<Scene>>,
    #[asset(paths(
        "scenes/Fox.glb#Scene0", 
        "scenes/main_character.glb#Scene0", 
        "scenes/electrician.glb#Scene0",
        "scenes/scientist.glb#Scene0"), collection(typed, mapped))]
    pub(crate) object: HashMap<String, Handle<Scene>>,
    #[asset(path = "scenes/college.glb#Scene0")]
    pub(crate) level: Handle<Scene>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct AnimationAssets {
    #[asset(paths(
        "scenes/Fox.glb#Animation0",
        "scenes/main_character.glb#Animation0", 
        "scenes/electrician.glb#Animation1",
        "scenes/scientist.glb#Animation0",
        "scenes/scientist.glb#Animation1",
        "scenes/scientist.glb#Animation2",
        "scenes/scientist.glb#Animation3"), collection(typed, mapped))]
    pub(crate) character_idle: HashMap<String, Handle<AnimationClip>>,
    #[asset(paths(
        "scenes/Fox.glb#Animation1", 
        "scenes/main_character.glb#Animation1", 
        "scenes/electrician.glb#Animation1",
        "scenes/scientist.glb#Animation1"), collection(typed, mapped))]
    pub(crate) character_walking: HashMap<String, Handle<AnimationClip>>,
    #[asset(paths(
        "scenes/Fox.glb#Animation2",
        "scenes/main_character.glb#Animation2",
        "scenes/electrician.glb#Animation1",
        "scenes/scientist.glb#Animation1"), collection(typed, mapped))]
    pub(crate) character_running: HashMap<String, Handle<AnimationClip>>,
    #[asset(paths(
        "scenes/Fox.glb#Animation2",
        "scenes/main_character.glb#Animation2",
        "scenes/electrician.glb#Animation1",
        "scenes/scientist.glb#Animation1"), collection(typed, mapped))]
    pub(crate) character_falling: HashMap<String, Handle<AnimationClip>>,
    #[asset(path = "scenes/main_character.glb#Animation1")]
    pub(crate) player_character_idle: Handle<AnimationClip>,
    #[asset(path = "scenes/main_character.glb#Animation3")]
    pub(crate) player_character_walking: Handle<AnimationClip>,
    #[asset(path = "scenes/main_character.glb#Animation2")]
    pub(crate) player_character_running: Handle<AnimationClip>,
    #[asset(path = "scenes/main_character.glb#Animation0")]
    pub(crate) player_character_aerial: Handle<AnimationClip>,
}


#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct ImageAssets {
    #[asset(paths("branding/banner.png", "branding/sky.jpg", "branding/bevy_logo_dark.png", "branding/bevy_logo_dark_big.png"), collection(typed, mapped))]
    // #[asset(path="branding", collection(typed, mapped))]
    pub(crate) images: HashMap<String, Handle<Image>>,
}









#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct LevelAssets {
    #[asset(paths("levels/old_town.lvl.ron"), collection(typed, mapped))]
    pub(crate) levels: HashMap<String, Handle<SerializedLevel>>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct TextureAssets {
    #[asset(path = "textures/stone_alley_2.jpg")]
    pub(crate) glowy_interior: Handle<Image>,
    #[asset(path = "textures/sky.jpg")]
    pub(crate) sky: Handle<Image>,
}

#[derive(AssetCollection, Resource, Clone)]
pub(crate) struct ConfigAssets {
    #[allow(dead_code)]
    #[asset(path = "config/config.game.toml")]
    pub(crate) game: Handle<GameConfig>,
}

fn show_progress(
    progress: Option<Res<ProgressCounter>>,
    mut egui_contexts: EguiContexts,
    mut last_done: Local<u32>,
    audio_assets: Option<Res<AudioAssets>>,
    scene_assets: Option<Res<SceneAssets>>,
    image_assets: Option<Res<ImageAssets>>,
    animation_assets: Option<Res<AnimationAssets>>,
    level_assets: Option<Res<LevelAssets>>,
    texture_assets: Option<Res<TextureAssets>>,
    config_assets: Option<Res<ConfigAssets>>,
) {
    if let Some(progress) = progress.map(|counter| counter.progress()) {
        if progress.done > *last_done {
            *last_done = progress.done;
        }

        egui::CentralPanel::default().show(egui_contexts.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(100.0);
                ui.heading("Loading");
                ui.label("Loading assets...");
                ui.add(
                    ProgressBar::new(progress.done as f32 / progress.total as f32).animate(true),
                );
                ui.add_space(100.0);
                ui.add_enabled_ui(false, |ui| {
                    ui.checkbox(&mut audio_assets.is_some(), "Audio");
                    ui.checkbox(&mut scene_assets.is_some(), "Scenes");
                    ui.checkbox(&mut animation_assets.is_some(), "Animations");
                    ui.checkbox(&mut level_assets.is_some(), "Levels");
                    ui.checkbox(&mut texture_assets.is_some(), "Textures");
                    ui.checkbox(&mut config_assets.is_some(), "Config");
                    ui.checkbox(&mut image_assets.is_some(), "Images");
                });
            });
        });
    }
}

#[sysfail(log(level = "error"))]
fn update_config(
    mut commands: Commands,
    config: Res<Assets<GameConfig>>,
    mut config_asset_events: EventReader<AssetEvent<GameConfig>>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("update_config").entered();
    for event in config_asset_events.read() {
        match event {
            AssetEvent::Modified { id } | AssetEvent::LoadedWithDependencies { id } => {
                // Guaranteed by Bevy to not fail
                let config = config.get(*id).unwrap();
                commands.insert_resource(config.clone());
            }
            _ => {}
        }
    }
    Ok(())
}
