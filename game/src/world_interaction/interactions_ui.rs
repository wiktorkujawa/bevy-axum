use crate::file_system_interaction::asset_loading::ImageAssets;
use crate::player_control::actions::{ActionsFrozen, PlayerAction};
use crate::player_control::camera::{
    // ForceCursorGrabMode,
    IngameCamera,
    IngameCameraKind,
};
use crate::player_control::player_embodiment::Player;
// use crate::util::criteria::is_frozen;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContexts};
use bevy_mod_sysfail::*;
use bevy_rapier3d::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

use super::dialog::Dialog;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum DialogOpened {
    #[default]
    Off,
    On,
}

pub(crate) fn interactions_ui_plugin(app: &mut App) {
    app.register_type::<InteractionOpportunities>()
        .init_resource::<InteractionOpportunities>()
        .add_state::<DialogOpened>()
        .add_systems(
            Update,
            (update_interaction_opportunities, update_interaction_ui)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(
            Update,
            display_interaction_prompt.run_if(resource_exists::<InteractionUi>()), // .run_if(resource_exists::<InteractionUi>().and_then(not(is_frozen)))
                                                                                   // .run_if(in_state(GameState::Playing)),
        );
}

#[derive(Resource, Debug)]
pub(crate) struct InteractionUi {
    source: Entity,
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
pub(crate) struct InteractionOpportunities(pub(crate) HashSet<Entity>);

fn update_interaction_opportunities(
    mut collision_events: EventReader<CollisionEvent>,
    player_query: Query<Entity, With<Player>>,
    parent_query: Query<&Parent>,
    mut interaction_opportunities: ResMut<InteractionOpportunities>,
) {
    for event in collision_events.read() {
        let (entity_a, entity_b, ongoing) = unpack_event(event);

        let (_player_entity, target_entity) =
            match determine_player_and_target(&player_query, &parent_query, entity_a, entity_b) {
                Some((dialog_source, dialog_target)) => (dialog_source, dialog_target),
                None => continue,
            };
        if ongoing {
            interaction_opportunities.0.insert(target_entity);
        } else {
            interaction_opportunities.0.remove(&target_entity);
        }
    }
}

#[sysfail(log(level = "error"))]
fn update_interaction_ui(
    mut commands: Commands,
    interaction_ui: Option<ResMut<InteractionUi>>,
    non_player_query: Query<&Transform, (Without<Player>, Without<IngameCamera>)>,
    player_query: Query<&Transform, (With<Player>, Without<IngameCamera>)>,
    interaction_opportunities: Res<InteractionOpportunities>,
    camera_query: Query<(&IngameCamera, &Transform), Without<Player>>,
) -> Result<()> {
    let mut valid_target = None;
    for entity in interaction_opportunities.0.iter() {
        let target_transform = non_player_query
            .get(*entity)
            .context("Failed to get transform of interaction target")?;
        // .context("Failed to get transform of interaction target").unwrap();
        for player_transform in player_query.iter() {
            for (camera, camera_transform) in camera_query.iter() {
                let is_facing_target = is_facing_target(
                    *player_transform,
                    *target_transform,
                    *camera_transform,
                    camera,
                );
                if is_facing_target {
                    valid_target = Some(*entity);
                    break;
                }
            }
        }
    }
    if let Some(mut interaction_ui) = interaction_ui {
        if let Some(valid_target) = valid_target {
            interaction_ui.source = valid_target;
        } else {
            commands.remove_resource::<InteractionUi>();
        }
    } else if let Some(valid_target) = valid_target {
        commands.insert_resource(InteractionUi {
            source: valid_target,
        });
    }

    Ok(())
}

fn unpack_event(event: &CollisionEvent) -> (Entity, Entity, bool) {
    match event {
        CollisionEvent::Started(entity_a, entity_b, _kind) => (*entity_a, *entity_b, true),
        CollisionEvent::Stopped(entity_a, entity_b, _kind) => (*entity_a, *entity_b, false),
    }
}

fn determine_player_and_target(
    player_query: &Query<Entity, With<Player>>,
    parent_query: &Query<&Parent>,
    entity_a: Entity,
    entity_b: Entity,
) -> Option<(Entity, Entity)> {
    if player_query.get(entity_a).is_ok() {
        let player_entity = entity_a;
        let target_entity = parent_query
            .get(entity_b)
            .map(|parent| parent.get())
            .unwrap_or(entity_b);
        Some((player_entity, target_entity))
    } else if player_query.get(entity_b).is_ok() {
        let player_entity = entity_b;
        let target_entity = parent_query
            .get(entity_a)
            .map(|parent| parent.get())
            .unwrap_or(entity_a);
        Some((player_entity, target_entity))
    } else {
        None
    }
}

fn is_facing_target(
    player_transform: Transform,
    target_transform: Transform,
    camera_transform: Transform,
    camera: &IngameCamera,
) -> bool {
    if camera.kind == IngameCameraKind::FixedAngle {
        return true;
    }
    let camera_to_player = camera_transform.forward();
    let player_to_target = target_transform.translation - player_transform.translation;
    let angle = camera_to_player.angle_between(player_to_target);
    angle < TAU / 8.
}

#[sysfail(log(level = "error"))]
fn display_interaction_prompt(
    // mut commands: Commands,
    // mut asset_server: Res<AssetServer>,
    interaction_ui: Res<InteractionUi>,
    mut actions_frozen: ResMut<ActionsFrozen>,
    // mut dialog_event_writer: EventWriter<DialogEvent>,
    mut egui_contexts: EguiContexts,
    actions: Query<&ActionState<PlayerAction>>,
    primary_windows: Query<&Window, With<PrimaryWindow>>,
    npc_query: Query<(Entity, &Dialog), With<Dialog>>,
    // force_cursor_grab: Res<ForceCursorGrabMode>,
    // mut dialog_query: Query<
    //     (&mut Style, &mut BackgroundColor, &mut BorderColor, Entity),
    //     With<GameObject>,
    // >,
    mut get_dialog_opened: Res<State<DialogOpened>>,
    mut change_dialog_opened: ResMut<NextState<DialogOpened>>,
    // mut interaction_query: Query<
    //     (
    //         &Interaction,
    //         // &Children,
    //         // // Entity,
    //         // &mut Button,
    //         &mut Element,
    //         &mut BackgroundColor,
    //     ),
    //     (
    //         //     // Changed<Interaction>,
    //         //     // With<Button>,
    //         Without<Dialog>,
    //         With<Element>,
    //         Without<GameObject>,
    //     ),
    // >,
    mut image_handles: Res<ImageAssets>,
) -> Result<()> {
    for actions in actions.iter() {
        let window = primary_windows
            .get_single()
            .context("Failed to get primary window")?;
        // .context("Failed to get primary window").unwrap();

        egui::Window::new("Interaction")
            .collapsible(false)
            .title_bar(false)
            .auto_sized()
            .fixed_pos(egui::Pos2::new(window.width() / 2., window.height() / 2.))
            .show(egui_contexts.ctx_mut(), |ui| {
                ui.label("E: Talk");
            });

        if let Ok((_entity, dialog)) = npc_query.get(interaction_ui.source) {
            dialog.open_dialog(
                &mut actions_frozen,
                actions,
                &mut get_dialog_opened,
                &mut change_dialog_opened,
                &mut egui_contexts,
                &mut image_handles,
            );
        }
    }
    Ok(())
}
