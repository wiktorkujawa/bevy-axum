use crate::level_instantiation::spawning::GameObject;

use bevy::prelude::*;

use super::SpawnStruct;

pub fn get_ui_spacing(array: Vec<f32>) -> UiRect {
    match array.len() {
        1 => UiRect::all(Val::Px(array[0])),
        2 => UiRect::axes(Val::Px(array[1]), Val::Px(array[0])),
        3 => UiRect::new(
            Val::Px(array[1]),
            Val::Px(array[1]),
            Val::Px(array[0]),
            Val::Px(array[2]),
        ),
        4 => UiRect::new(
            Val::Px(array[3]),
            Val::Px(array[1]),
            Val::Px(array[0]),
            Val::Px(array[2]),
        ),
        _ => UiRect::all(Val::Px(0.0)),
    }
}

pub(crate) fn spawn(In(SpawnStruct { .. }): In<SpawnStruct>, mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                // margin: UiRect::all(Val::Auto),
                margin: UiRect::new(Val::Auto, Val::Auto, Val::Auto, Val::Px(0.0)),
                // width: default_width(),
                width: Val::Percent(100.0),
                // height: default_height(),
                height: Val::Percent(50.0),
                flex_wrap: FlexWrap::Wrap,
                flex_direction: FlexDirection::Column,
                display: Display::None,
                // display: Display::Flex,
                // padding: get_ui_spacing(default_padding()),
                padding: get_ui_spacing(vec![0.0]),
                row_gap: Val::Px(20.0),
                column_gap: Val::Px(20.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                // border: get_ui_spacing(default_border()),
                border: get_ui_spacing(vec![0.0]),
                ..default()
            },
            // border_color: BorderColor(default_border_color()),
            border_color: BorderColor(Color::BLACK),
            // background_color: BackgroundColor(default_background_color()),
            background_color: BackgroundColor(Color::default()),
            ..default()
        },
        Name::new("Dialog"),
        GameObject::Dialog,
    ));
}
