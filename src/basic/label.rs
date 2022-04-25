use bevy_asset::AssetServer;
use bevy_ecs::prelude::*;
use bevy_hierarchy::ChildBuilder;
use bevy_render::color::Color;
use bevy_text::{Text, TextStyle};
use bevy_ui::prelude::*;

pub fn generate_label(
    text: &str,
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) -> Entity {
    parent
        .spawn_bundle(TextBundle {
            text: Text::with_section(
                text,
                TextStyle {
                    font: asset_server.load("fonts/monogram-extended.ttf"),
                    font_size: 40.0,
                    color: Color::rgb(0.9, 0.9, 0.9),
                },
                Default::default(),
            ),
            ..Default::default()
        })
        .id()
}
