use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::window::WindowMode;

/// Overrides the default Bevy plugins and configures things like the screen settings.
pub(crate) fn bevy_config_plugin(app: &mut App) {
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resolution: (1920., 1080.).into(),
            title: "Foxtrot".to_string(),
            canvas: Some("#bevy".to_owned()),
            present_mode: PresentMode::AutoVsync,
            mode: WindowMode::BorderlessFullscreen,
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    });
    app.add_plugins(default_plugins);
}
