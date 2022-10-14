use bevy::{prelude::*, time::FixedTimestep};
use bevy_ecs_ldtk::prelude::*;
use rand::prelude::*;

const TETRIS_TICK_MS: f64 = 150.0;

pub struct BlockPlugin;

impl Plugin for BlockPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NewStockEvent>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(FixedTimestep::step(TETRIS_TICK_MS / 60.0))
                    .with_system(|mut e: EventWriter<NewStockEvent>| {
                        e.send(NewStockEvent);
                    })
                    .with_system(block_gravity),
            )
            .register_ldtk_entity::<BlockBundle>("T")
            .register_ldtk_entity::<BlockBundle>("S")
            .register_ldtk_entity::<BlockBundle>("Z")
            .register_ldtk_entity::<BlockBundle>("L")
            .register_ldtk_entity::<BlockBundle>("J")
            .register_ldtk_entity::<BlockBundle>("Line")
            .register_ldtk_entity::<BlockBundle>("Square")
            .add_system(block_spawning)
            .add_system(block_rotation)
            .add_system(update_block_transform);
    }
}

struct NewStockEvent;

#[derive(Bundle, LdtkEntity)]
pub struct BlockBundle {
    block: Block,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_sheet: SpriteSheetBundle,
    #[grid_coords]
    coords: GridCoords
}

#[derive(Component, Default)]
struct Block;

fn block_spawning(
    mut commands: Commands,
    mut new_stock_events: EventReader<NewStockEvent>,
    asset_server: Res<AssetServer>,
) {
    if new_stock_events.iter().next().is_some() {
        todo!()
    }
}

fn block_rotation(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Block>>) {
    let angle = if keys.just_pressed(KeyCode::Left) {
        0.25
    } else if keys.just_pressed(KeyCode::Right) {
        0.75
    } else {
        0.0
    } * std::f32::consts::TAU;

    for mut block_transform in &mut query {
        block_transform.rotate_z(angle);
    }
}

const BLOCK_GRAVITY: i32 = 1;

fn block_gravity(mut query: Query<&mut GridCoords, With<Block>>) {
    for mut block_coords in &mut query {
        block_coords.y -= BLOCK_GRAVITY;
    }
}

fn update_block_transform(mut query: Query<(&GridCoords, &mut Transform), With<Block>>) {
    for (block_coords, mut block_transform) in &mut query {
        block_transform.translation.x = (block_coords.x * 16) as f32;
        block_transform.translation.y = (block_coords.y * 16) as f32;
    }
}
