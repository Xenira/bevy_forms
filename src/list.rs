use std::{
    fmt,
    sync::{Arc, Mutex},
};

use bevy_app::Plugin;
use bevy_asset::AssetServer;
use bevy_ecs::{prelude::*, system::EntityCommands};
use bevy_hierarchy::{BuildChildren, ChildBuilder, Children, DespawnRecursiveExt};
use bevy_input::mouse::{MouseScrollUnit, MouseWheel};
use bevy_math::Size;
use bevy_render::color::Color;
use bevy_ui::{prelude::*, FocusPolicy};
use tracing::warn;
use uuid::Uuid;

use crate::{form::FormId, theme::Theme};

pub trait FormListItem: Send + Sync + fmt::Debug {
    fn get_key(&self) -> &str;
    fn spawn(&self, commands: &EntityCommands);
}

#[derive(Component, Debug)]
pub struct FormListComponent {
    id: Uuid,
    position: f32,
    items: Vec<Box<dyn FormListItem>>,
}

impl Default for FormListComponent {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            position: Default::default(),
            items: Default::default(),
        }
    }
}

#[derive(Component, Default)]
pub struct FormListItemComponent {
    pub key: String,
}

impl PartialEq<dyn FormListItem> for FormListItemComponent {
    fn eq(&self, other: &dyn FormListItem) -> bool {
        self.key == other.get_key()
    }
}

impl FormListItemComponent {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
        }
    }
}

#[derive(Bundle)]
pub struct FormListBundle {
    pub id: FormId,
    list: FormListComponent,

    #[bundle]
    node: NodeBundle,
}

pub struct FormListPlugin;

impl Plugin for FormListPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        app.add_system(scroll_events).add_system(detect_removals);
    }
}

pub fn generate_list(
    id: &str,
    asset_server: &Res<AssetServer>,
    theme: Option<Theme>,
    commands: &mut Commands,
    parent: &mut ChildBuilder,
) -> Entity {
    parent
        .spawn_bundle(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::ColumnReverse,
                flex_grow: 1.0,
                overflow: Overflow::Hidden,
                ..Default::default()
            },
            color: Color::rgb(0.10, 0.10, 0.10).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(FormListBundle {
                node: NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::ColumnReverse,
                        // horizontally stretch
                        align_items: AlignItems::Stretch,
                        // vertically allign at top
                        justify_content: JustifyContent::FlexStart,
                        ..Default::default()
                    },
                    color: theme.unwrap_or(Theme::default()).colors.primary.into(),
                    focus_policy: FocusPolicy::Block,
                    ..Default::default()
                },
                id: FormId(id.to_string()),
                list: FormListComponent::default(),
            });
        })
        .id()
}

fn detect_removals(
    removals: RemovedComponents<FormListItemComponent>,
    // ... (maybe Commands or a Query ?) ...
) {
    for entity in removals.iter() {
        warn!("Form list item was removed from entity {:?}", entity);
    }
}

fn list_change(
    mut commands: Commands,
    query: Query<(&FormListComponent, Entity, &Children), (Changed<FormListComponent>)>,
    q_list_item: Query<&FormListItemComponent>,
) {
    for (list, entity, children) in query.iter() {
        let mut child_index = 0;
        for (i, item) in list.items.iter().enumerate() {
            let child = children[child_index];

            if let Ok(list_item) = q_list_item.get(child) {
                child_index += 1;
                if list_item == item.as_ref() {
                    continue;
                } else {
                }
            }
        }
        // for child in children.iter() {
        //     index += 1;
        //     if let Ok(item) = q_list_item.get(*child) {
        //         if item == list.items[index].as_ref() {
        //             continue;
        //         }
        //         let new_item = commands
        //             .spawn()
        //             .insert(FormListItemComponent::new(list.items[index]))
        //             .id();
        //         commands.entity(entity).insert_children(index, &[new_item]);
        // 		commands.entity(entity).
        //     }
        // }
    }
}

fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut interaction_query: Query<(
        &mut FormListComponent,
        &mut Style,
        Option<&Interaction>,
        &Children,
        &Node,
    )>,
    query_item: Query<&Node>,
) {
    for ev in scroll_evr.iter() {
        for (mut list, mut style, interaction, children, uinode) in interaction_query.iter_mut() {
            if let Some(interaction) = interaction {
                if *interaction == Interaction::None {
                    continue;
                }
            }
            let items_height: f32 = children
                .iter()
                .map(|entity| query_item.get(*entity).unwrap().size.y)
                .sum();
            let panel_height = uinode.size.y;
            let max_scroll = (items_height - panel_height).max(0.);
            let dy = match ev.unit {
                MouseScrollUnit::Line => ev.y * 20.,
                MouseScrollUnit::Pixel => ev.y,
            };
            list.position += dy;
            list.position = list.position.clamp(-max_scroll, 0.);
            style.position.top = Val::Px(list.position);
        }
    }
}
