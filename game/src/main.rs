// use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
// use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::prelude::*;
use game::GamePlugin;
// use foxtrot::GamePlugin;
// use bevy::window::{MonitorSelection, WindowPosition};


// Resource for fonts:
#[derive(Default, Clone, Resource)]
pub struct ResFont {
    pub ui: Handle<Font>,
}

// Load resource during startup:
// pub fn startup(asset_server: Res<AssetServer>, mut res_font: ResMut<ResFont>)
// {
//     res_font.ui = asset_server.load("/bevy-assets/Roboto-Black.ttf");
// }

fn main() {
    App::new()
    .add_plugins(GamePlugin).run();
        // .insert_resource(ResFont::default())
        // .add_plugins((
        //     DefaultPlugins.set(WindowPlugin {
        //         primary_window: Some(Window {
        //             prevent_default_event_handling: false,
        //             canvas: Some("#canvas".to_string()),
        //             position: WindowPosition::Centered(MonitorSelection::Current),
        //             ..default()
        //         }),
        //         ..default()
        //     }),
        //     LogDiagnosticsPlugin::default(),
        //     FrameTimeDiagnosticsPlugin,
        // ))
        // .add_systems(Startup, (setup,startup))
        // .add_systems(Update, main_menu)
        // .run();
}

// fn setup(mut commands: Commands) {
//     commands.spawn(Camera2dBundle::default());
// }

// fn main_menu(mut commands: Commands, assets: Res<ResFont>) {
//     let font = assets.ui.clone();

//     let title_font = title_text_style(60.0, font.clone());

//     commands.spawn({
//         Text2dBundle {
//             text: Text::from_section("Hello world!", title_font.clone())
//                 .with_justify(JustifyText::Center),
//             ..default()
//         }
//     });
// }

// fn title_text_style(font_size: f32, font: Handle<Font>) -> TextStyle {
//     TextStyle {
//         font,
//         font_size,
//         color: Color::WHITE,
//     }
// }
