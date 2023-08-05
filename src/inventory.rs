use bevy::prelude::*;
use bevy_rapier2d::{prelude::{RapierContext, QueryFilter, CollisionGroups, Group}, rapier::prelude::InteractionGroups};

use crate::game::{MainCamera, Player, GameState, setup_player};

#[derive(Clone, Copy, Debug)]
pub enum InventoryItemType {
    Consumable,
    Equipment,
}

#[derive(Clone, Debug)]
pub struct Item {
    pub name: String,
    pub item_type: InventoryItemType,
}

#[derive(Component)]
pub struct ItemOnGround {
    pub item: Item,
}

#[derive(Default, Component)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub inventory_size: usize,
}

#[derive(Component)]
struct InventoryUi;

#[derive(Component)]
struct InventoryItemUi;

fn system_setup_inventory(
    mut commands: Commands,
    player_q: Query<Entity, With<Player>>,
) {
    let Ok(entity) = player_q.get_single() else {
        println!("Player not found");
        return;
    };

    commands.entity(entity).insert(Inventory::default());

    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Px(100.),
                position_type: PositionType::Absolute,
                bottom: Val::Px(0.),
                left: Val::Px(0.),
                justify_content: JustifyContent::Center,

                display: Display::Grid,
                grid_template_columns: vec![
                    GridTrack::min_content(),
                    GridTrack::min_content(),
                    GridTrack::min_content(),
                ],
                grid_template_rows: vec![GridTrack::auto()],
                ..default()
            },
            ..default()
        },
        InventoryUi,
    ))
        .with_children(|builder| {
            spawn_inventory_item(builder, "Nothing");
            spawn_inventory_item(builder, "Nothing");
            spawn_inventory_item(builder, "Nothing");
        });
}

fn spawn_inventory_item(builder: &mut ChildBuilder, text: &str) {
    builder.spawn((
        NodeBundle {
            style: Style {
                display: Display::Grid,
                width: Val::Px(100.),
                height: Val::Px(100.),
                ..default()
            },
            background_color: Color::rgb(0.65, 0.65, 0.65).into(),
            ..default()
        },
        InventoryItemUi,
    ))
        .with_children(|builder| {
            spawn_nested_text_bundle(builder, text);
        });
}

fn spawn_nested_text_bundle(builder: &mut ChildBuilder, text: &str) {
    builder.spawn(TextBundle::from_section(
        text,
        TextStyle {
            font_size: 24.0,
            color: Color::BLACK,
            ..default()
        },
    ));
}

fn system_update_inventory(
    inventory_q: Query<&Children, With<InventoryItemUi>>,
    mut text_q: Query<&mut Text>,
    player_q: Query<&Inventory, With<Player>>,
) {
    let Ok(inventory) = player_q.get_single() else {
        return;
    };
    
    for (index, children) in inventory_q.iter().enumerate() {
        if let Some(item) = inventory.items.get(index) {
            if let Some(text) = children.get(0) {
                if let Ok(mut text) = text_q.get_mut(*text) {
                    text.sections[0].value = item.name[..].into();
                }
            }
        }
    }
}

fn system_inventory_pickup(
    mut commands: Commands,
    windows: Query<&Window>,
    cam_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mouse: Res<Input<MouseButton>>,
    item_query: Query<(Entity, &Transform, &ItemOnGround), Without<Player>>,
    mut player_query: Query<(Entity, &Transform, &mut Inventory), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    if !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    (|| {
        let wnd = windows.single();

        let (camera, camera_transform) = cam_query.get_single().ok()?;

        let mouse_pos_2d = wnd.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))?;

        let (player, player_transform, mut inventory) = player_query.single_mut();

        let filter = QueryFilter::new().groups(
            CollisionGroups::new(Group::GROUP_2, Group::GROUP_2),
        );
        if let Some((entity, projection)) = rapier_context.project_point(
            mouse_pos_2d,
            true,
            filter,
        ) {
            let (item, transform, item_on_ground) = item_query.get(entity).ok()?;
            inventory.items.push(item_on_ground.item.clone());
            commands.entity(item).despawn();
            
            return Some(());
        }
        //for (item_entity, item_transform, item_on_ground) in item_query.iter() {
        //}

        Some(())
    })();
}

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::InGame), (
                apply_deferred,
                system_setup_inventory,
            ).chain().after(setup_player))
            .add_systems(Update, (
                system_inventory_pickup,
                system_update_inventory,
            ));
    }
}
