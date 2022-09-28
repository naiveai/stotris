use bevy::{
    log::{Level, LogSettings},
    prelude::*,
    time::FixedTimestep,
};
use rand::prelude::*;

const TETRIS_TICK_MS: f64 = 150.0;

fn main() {
    let mut app = App::new();

    #[cfg(debug_assertions)]
    app.insert_resource(LogSettings {
        filter: "warn,stotris=debug".into(),
        level: Level::DEBUG,
    });

    #[cfg(not(debug_assertions))]
    app.insert_resource(LogSettings {
        filter: "warn".into(),
        level: Level::WARN,
    });

    app.insert_resource(WindowDescriptor {
        title: "Stotris".to_string(),
        width: 500.0,
        height: 500.0,
        ..default()
    })
    .add_plugins(DefaultPlugins);

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());

    app.add_startup_system(setup)
        .add_startup_system(setup_buckets)
        .add_event::<NewStockEvent>()
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
        .add_system(block_despawning)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    debug!("Spawning camera");
    commands.spawn_bundle(Camera2dBundle::default());
}

#[derive(Component)]
struct Bucket;

fn setup_buckets(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!("Spawning buckets");

    let bucket_image = asset_server.load("bucket.png");

    for x_corner in [-140.0, -40.0, 60.0, 160.0] {
        commands
            .spawn_bundle(SpriteBundle {
                texture: bucket_image.clone(),
                transform: Transform {
                    translation: Vec3::new(x_corner, -200.0, 0.0),
                    scale: Vec3::new(3.0, 3.0, 1.0),
                    ..default()
                },
                ..default()
            })
            .insert(Bucket);
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
                transform: Transform {
                    translation: new_block_position,
                    ..default()
                },
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

const BLOCK_GRAVITY: f32 = 30.0;

fn block_gravity(mut query: Query<&mut Transform, With<Block>>) {
    for mut block_transform in &mut query {
        block_transform.translation.y -= BLOCK_GRAVITY;
    }
}

fn block_rotation(keys: Res<Input<KeyCode>>, mut query: Query<&mut Transform, With<Block>>) {
    use std::f32::consts::TAU;

    let rotation = if keys.just_pressed(KeyCode::Left) {
        Quat::from_rotation_z(TAU / 4.0)
    } else if keys.just_pressed(KeyCode::Right) {
        Quat::from_rotation_z(0.75 * TAU)
    } else {
        Quat::IDENTITY
    };

    for mut block_transform in &mut query {
        block_transform.rotation *= rotation;
    }
}
