use bevy::{prelude::*, time::FixedTimestep};
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
            .add_system(block_spawning)
            .add_system(block_rotation)
            .add_system(block_despawning);
    }
}

struct NewStockEvent;

#[derive(Component)]
struct Block;

fn block_spawning(
    mut commands: Commands,
    mut new_stock_events: EventReader<NewStockEvent>,
    asset_server: Res<AssetServer>,
) {
    let block_image = asset_server.load("block.png");

    if new_stock_events.iter().next().is_some() {
        let new_block_position = Vec3::new(rand::thread_rng().gen_range(-150.0..150.0), 200.0, 0.0);

        debug!("Spawning block at {:?}", new_block_position);

        commands
            .spawn_bundle(SpriteBundle {
                texture: block_image,
                transform: Transform::from_translation(new_block_position),
                ..default()
            })
            .insert(Block);
    }
}

fn block_despawning(mut commands: Commands, query: Query<(Entity, &Transform), With<Block>>) {
    for (block, position) in &query {
        if position.translation.y <= -300.0 {
            debug!("Despawning block at {:?}", position.translation);
            commands.entity(block).despawn();
        }
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

const BLOCK_GRAVITY: f32 = 30.0;

fn block_gravity(mut query: Query<&mut Transform, With<Block>>) {
    for mut block_transform in &mut query {
        block_transform.translation.y -= BLOCK_GRAVITY;
    }
}
