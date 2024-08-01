pub(crate) use crate::world_interaction::dialog::resources::{
    CurrentDialog, 
    // DialogId
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use bevy_egui::{
    egui::{self, Color32, FontId, Margin, RichText, Rounding, ScrollArea},
    EguiContexts,
    EguiPlugin
};
use leafwing_input_manager::action_state::ActionState;
use crate::{
    player_control::actions::{ActionsFrozen, PlayerAction},
    world_interaction::interactions_ui::DialogOpened,
};
use crate::file_system_interaction::asset_loading::ImageAssets;

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


#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub struct Dialog {
    #[serde(default = "default_sections")]
    pub sections: Vec<Section>,
    #[serde(default = "default_settings")]
    pub dialog_settings: DialogSettings,
}

impl Default for Dialog {
    fn default() -> Self {
        Self {
            sections: default_sections(),
            dialog_settings: default_settings(),
        }
    }
}

impl Dialog {
    pub fn open_dialog(
        &self,
        actions_frozen: &mut ResMut<ActionsFrozen>,
        actions: &ActionState<PlayerAction>,
        get_dialog_opened: &mut Res<State<DialogOpened>>,
        change_dialog_opened: &mut ResMut<NextState<DialogOpened>>,
        egui_contexts: &mut EguiContexts,
        image_handles: &mut Res<ImageAssets>,
    ) {
            match get_dialog_opened.get() {
                DialogOpened::Off => {
                    // actions_frozen.unfreeze();
                }
                DialogOpened::On => {

                    let Dialog {
                        dialog_settings,
                        sections,
                        ..
                    } = self;

                    let ctx = egui_contexts.ctx_mut().clone();

                    egui::CentralPanel::default()
                        .frame(egui::Frame {
                            fill: egui::Color32::from_black_alpha(240),
                            inner_margin: get_ui_spacing(dialog_settings.padding.clone()),
                            outer_margin: Margin::same(20.0),
                            rounding: Rounding::same(20.0),
                            ..default()
                        })
                        .show(&ctx, |ui| {
                            egui::TopBottomPanel::top("top_panel").frame(egui::Frame {
                                fill: egui::Color32::TRANSPARENT, // Make the panel's background transparent
                                inner_margin: Margin::symmetric(4.0, 4.0),
                                ..default()
                            })
                            .show_inside(ui, |ui| {
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
                                    if ui.button("X").clicked() {
                                        change_dialog_opened.set(DialogOpened::Off);
                                        actions_frozen.unfreeze();
                                    }
                                });
                            });
                            
                            let max_rect = ui.max_rect();
                            let max_size = max_rect.size();
                            egui::Grid::new("Dialog").show(ui, |ui| {
                                for section in sections {
                                    // ui.vertical_centered_justified(|ui| {
                                    let element_size = section.elements.len();

                                    // println!("NUmber of elements {}", element_size);
                                    for element in &section.elements {
                                        // ui.horizontal(|ui| {
                                        match element.element_type {
                                            ElementType::Text => {
                                                // ui.label(element.text_sections[0].text.clone());
                                                // ui.vertical(|ui| {
                                                egui::Grid::new("text_section").show(ui, |ui| {
                                                    // egui::ScrollArea::vertical().max_width(100.0).min_scrolled_height(200.0).show(ui, |ui| {
                                                    // let window_size = ui.available_size();
                                                    // let max_rect = ui.max_rect();
                                                    // let max_size = max_rect.size();

                                                    ui.vertical(|ui| {
                                                        ui.set_min_width(
                                                            max_size.x / (element_size as f32),
                                                        );
                                                        
                                                        for text_section in &element.text_sections {
                                                            let transform_color = bevy_egui::egui::Color32::from_rgba_unmultiplied(
                                                                (text_section.color.r() * 255.0).round() as u8,
                                                                (text_section.color.g() * 255.0).round() as u8,
                                                                (text_section.color.b() * 255.0).round() as u8,
                                                                (text_section.color.a() * 255.0).round() as u8,
                                                            );
                                                            ui.label(
                                                                RichText::new(
                                                                    text_section.text.clone(),
                                                                )
                                                                .color(transform_color)
                                                                .font(FontId::proportional(
                                                                    text_section.font_size,
                                                                )),
                                                            );

                                                            ui.end_row();
                                                        }
                                                    });
                                                });

                                                // RichText::new("Large text").font(FontId::proportional(40.0))
                                            }
                                            ElementType::Image => {
                                                let rendered_texture_id = egui_contexts.add_image(
                                                    image_handles
                                                        .images
                                                        .get(element.image.as_str())
                                                        .unwrap()
                                                        .clone_weak(),
                                                );
                                                

                                                ui.add(egui::widgets::Image::new(
                                                    egui::load::SizedTexture::new(
                                                        rendered_texture_id,
                                                        element.image_size,
                                                    ),
                                                ));
                                            }
                                            ElementType::Button => {
                                                if ui.button(element.button.text.clone()).clicked()
                                                {
                                                    match element
                                                        .button
                                                        .interaction
                                                        .interaction_type
                                                    {
                                                        InteractionType::Link => {
                                                            if webbrowser::open(
                                                                element
                                                                    .button
                                                                    .interaction
                                                                    .link
                                                                    .as_str(),
                                                            )
                                                            .is_ok()
                                                            {
                                                                // actions_frozen.unfreeze();
                                                            }
                                                        }
                                                        InteractionType::Close => {
                                                            change_dialog_opened
                                                                .set(DialogOpened::Off);
                                                            actions_frozen.unfreeze();
                                                        }
                                                    }

                                                    // if webbrowser::open("https://just-dev-it.com/")
                                                    //         .is_ok()
                                                    //     {
                                                    // change_dialog_opened.set(DialogOpened::Off);
                                                    // actions_frozen.unfreeze();
                                                    // app_exit_events.send(AppExit);
                                                    // }
                                                }
                                            }
                                        }

                                        // });
                                    }
                                    ui.end_row();

                                    // });
                                }
                            });

                        });
                }
            }

            if actions.just_pressed(PlayerAction::Interact) {
                change_dialog_opened.set(DialogOpened::On);
                actions_frozen.freeze();
            }

    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub struct Section {
    pub elements: Vec<Element>,
}

#[derive(Component, Debug, Clone, Copy, Serialize, PartialEq, Deserialize, Reflect)]
pub enum ElementType {
    Text,
    Image,
    Button,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub struct Element {
    pub element_type: ElementType,
    #[serde(default = "default_text_sections")]
    pub text_sections: Vec<DialogText>,
    #[serde(default = "default_background")]
    pub background: Color,
    #[serde(default = "default_button")]
    pub button: DialogButton,
    #[serde(default = "default_image")]
    pub image: String,
    #[serde(default = "default_image_size")]
    pub image_size: [f32; 2],
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Reflect, Default)]
pub enum DialogLayout {
    #[default]
    Center,
    Bottom,
    FullScreen,
    Custom,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Reflect, Default)]
pub struct DialogSettings {
    #[serde(default = "default_layout")]
    pub layout: DialogLayout,
    #[serde(default = "default_background_color")]
    pub background_color: Color,
    #[serde(default = "default_border_color")]
    pub border_color: Color,
    #[serde(default = "default_width")]
    pub width: Val,
    #[serde(default = "default_height")]
    pub height: Val,
    #[serde(default = "default_padding")]
    pub padding: Vec<f32>,
    #[serde(default = "default_border")]
    pub border: Vec<f32>,
}

fn default_layout() -> DialogLayout {
    DialogLayout::Center
}

fn default_width() -> Val {
    Val::Percent(100.0)
}

fn default_height() -> Val {
    Val::Percent(50.0)
}

fn default_background_color() -> Color {
    Color::default()
}
fn default_border_color() -> Color {
    Color::BLACK
}

fn default_border() -> Vec<f32> {
    vec![0.0]
}

fn default_padding() -> Vec<f32> {
    vec![0.0]
}

fn default_settings() -> DialogSettings {
    DialogSettings {
        layout: default_layout(),
        height: default_height(),
        width: default_width(),
        background_color: default_background_color(),
        border_color: default_border_color(),
        border: default_border(),
        padding: default_padding(),
    }
}

fn default_image() -> String {
    String::from("branding/sky.jpg")
}

fn default_image_size() -> [f32; 2] {
    [100.0, 100.0]
}

fn default_text_sections() -> Vec<DialogText> {
    vec![]
}

fn default_sections() -> Vec<Section> {
    vec![]
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub struct DialogText {
    #[serde(default = "default_text")]
    pub text: String,
    #[serde(default = "default_font_size")]
    pub font_size: f32,
    #[serde(default = "default_font")]
    pub font: String,
    #[serde(default = "default_color")]
    pub color: Color,
}

fn default_text() -> String {
    String::from("")
}

fn default_font_size() -> f32 {
    12.0
}

fn default_font() -> String {
    String::from("fonts/FiraSans-Bold.ttf")
}

fn default_color() -> Color {
    Color::WHITE
}

fn default_background() -> Color {
    Color::NONE
}

fn default_button_border() -> Vec<f32> {
    vec![2.0]
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub struct DialogButton {
    #[serde(default = "default_button_text")]
    pub text: String,
    #[serde(default = "default_button_width")]
    pub width: Val,
    #[serde(default = "default_button_height")]
    pub height: Val,
    #[serde(default = "default_font_size")]
    pub font_size: f32,
    #[serde(default = "default_font")]
    pub font: String,
    #[serde(default = "default_button_font_color")]
    pub color: Color,
    #[serde(default = "default_button_background")]
    pub background: Color,
    #[serde(default = "default_border_color")]
    pub border_color: Color,
    #[serde(default = "default_hover_button")]
    pub hover: ButtonColor,
    #[serde(default = "default_button_border")]
    pub border: Vec<f32>,
    #[serde(default = "default_button_padding")]
    pub padding: Vec<f32>,
    #[serde(default = "default_button_interaction")]
    pub interaction: ButtonInteraction,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub enum InteractionType {
    Link,
    Close,
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub struct ButtonInteraction {
    pub interaction_type: InteractionType,
    pub link: String,
}

fn default_button_interaction() -> ButtonInteraction {
    ButtonInteraction {
        interaction_type: InteractionType::Link,
        link: String::from(""),
    }
}

fn default_button_text() -> String {
    String::from("Click me")
}

fn default_button_width() -> Val {
    Val::Auto
}

fn default_button_height() -> Val {
    Val::Px(40.0)
}

fn default_button_font_color() -> Color {
    Color::BLACK
}

fn default_button_background() -> Color {
    Color::RED
}

fn default_button_padding() -> Vec<f32> {
    vec![8.0, 16.0]
}

fn default_button() -> DialogButton {
    DialogButton {
        text: default_button_text(),
        width: default_button_width(),
        height: default_button_height(),
        font_size: default_font_size(),
        font: default_font(),
        color: default_button_font_color(),
        background: default_button_background(),
        border_color: default_border_color(),
        hover: default_hover_button(),
        border: default_border(),
        padding: default_button_padding(),
        interaction: default_button_interaction(),
    }
}

#[derive(Component, Debug, Clone, Serialize, Deserialize, PartialEq, Reflect)]
pub struct ButtonColor {
    #[serde(default = "default_button_font_color")]
    pub color: Color,
    #[serde(default = "default_button_background")]
    pub background: Color,
    #[serde(default = "default_border_color")]
    pub border_color: Color,
}

fn default_hover_button() -> ButtonColor {
    ButtonColor {
        color: default_button_font_color(),
        background: default_button_background(),
        border_color: default_border_color(),
    }
}

pub fn get_ui_spacing(array: Vec<f32>) -> Margin {
    match array.len() {
        1 => Margin::same(array[0]),
        2 => Margin::symmetric(array[1], array[0]),
        3 => Margin {
            top: array[0],
            right: array[1],
            bottom: array[2],
            left: array[1]
        },
        4 => Margin {
            top: array[0],
            right: array[1],
            bottom: array[2],
            left: array[3],
        },
        _ => Margin::same(0.0),
    }
}