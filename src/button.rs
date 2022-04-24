use bevy_asset::AssetServer;
use bevy_ecs::prelude::*;
use bevy_hierarchy::{BuildChildren, ChildBuilder};
use bevy_input::{mouse::MouseButton, Input};
use bevy_math::{Rect, Size};
use bevy_render::color::Color;
use bevy_text::{Text, TextStyle};
use bevy_ui::prelude::*;

use crate::{form::FormId, theme::Theme};

#[derive(Debug)]
pub struct ButtonClickEvent(pub String);

pub fn generate_button(
    text: &str,
    id: &str,
    asset_server: &Res<AssetServer>,
    theme: Option<Theme>,
    parent: &mut ChildBuilder,
) -> Entity {
    parent
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                // center button
                margin: Rect::all(Val::Auto),
                // horizontally center child text
                justify_content: JustifyContent::Center,
                // vertically center child text
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: theme.unwrap_or(Theme::default()).colors.button.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.9, 0.9, 0.9),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });
        })
        .insert(FormId(id.to_string()))
        .id()
}

pub fn button_system(
    theme: Res<Theme>,
    mut interaction_query: Query<
        (&Interaction, &FormId, &mut UiColor),
        (Changed<Interaction>, With<Button>),
    >,
    mouse_buttons: Res<Input<MouseButton>>,
    mut ev_button_click: EventWriter<ButtonClickEvent>,
) {
    for (interaction, id, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *color = theme.colors.button_clicked.into();
            }
            Interaction::Hovered => {
                *color = theme.colors.button_hovered.into();
                if mouse_buttons.just_released(MouseButton::Left) {
                    ev_button_click.send(ButtonClickEvent(id.0.to_string()));
                }
            }
            Interaction::None => {
                *color = theme.colors.button.into();
            }
        }
    }
}
