use bevy_ecs::prelude::*;
use bevy_forms_macros::enum_functions;
use bevy_render::color::Color;
use bevy_ui::prelude::*;

#[derive(Default)]
struct FlexLayout {
    style: Style,
    color: Color,
}

impl FlexLayout {}

#[enum_functions]
pub fn flex(justify_content: JustifyContent) -> NodeBundle {
    NodeBundle {
        style: Style {
            flex_grow: 1.0,
            flex_direction: FlexDirection::ColumnReverse,
            justify_content,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        color: Color::NONE.into(),
        ..Default::default()
    }
}
