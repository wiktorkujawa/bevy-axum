use bevy::prelude::*;
use game::GamePlugin;

// Resource for fonts:
#[derive(Default, Clone, Resource)]
pub struct ResFont {
    pub ui: Handle<Font>,
}

fn main() {
    App::new()
    .add_plugins(GamePlugin).run();
}