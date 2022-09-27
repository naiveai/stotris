use bevy::{prelude::*, time::FixedTimestep};

const TETRIS_TICK_MS: f64 = 150.0;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Stotris".to_string(),
        width: 350.0,
        height: 500.0,
        ..default()
    })
    .add_plugins(DefaultPlugins);

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());

    app.add_startup_system(setup)
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TETRIS_TICK_MS / 60.0))
                .with_system(block_gravity),
        )
        .add_system(block_rotation)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    info!("Spawning camera");
    commands.spawn_bundle(Camera2dBundle::default());

    let block_image = asset_server.load("block.png");

    info!("Spawning block");
    commands
        .spawn_bundle(SpriteBundle {
            texture: block_image.clone(),
            ..default()
        })
        .insert(Block);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(-30.0, 0.0, 0.0),
            texture: block_image.clone(),
            ..default()
        })
        .insert(Block);

    commands
        .spawn_bundle(SpriteBundle {
            transform: Transform::from_xyz(30.0, 0.0, 0.0),
            texture: block_image,
            ..default()
        })
        .insert(Block);
}

#[derive(Component)]
struct Block;

const BLOCK_GRAVITY: f32 = 10.0;

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
