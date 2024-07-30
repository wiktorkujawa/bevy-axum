pub(crate) use crate::world_interaction::dialog::resources::{
    CurrentDialog, 
    // DialogId
};
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
// use serde::{Deserialize, Serialize};

mod resources;

pub(crate) fn dialog_plugin(app: &mut App) {
    app.add_plugins(EguiPlugin);
        // .register_type::<DialogId>()
        // .add_event::<DialogEvent>();
        // .add_systems(
        //     Update,
        //     (set_current_dialog, show_dialog).run_if(in_state(GameState::Playing)),
        // );
}
