mod block;

use bevy::{
    log::{Level, LogSettings},
    prelude::*,
};
use block::BlockPlugin;

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
        .add_plugin(BlockPlugin)
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
