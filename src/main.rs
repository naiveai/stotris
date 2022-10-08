mod block;

use bevy::{
    log::{Level, LogSettings},
    prelude::*, render::camera::WindowOrigin,
};
use bevy_ecs_ldtk::prelude::*;
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
        width: 512.0,
        height: 640.0,
        ..default()
    })
    .add_plugins(DefaultPlugins);

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());

    app.add_startup_system(setup)
        .insert_resource(LevelSelection::Identifier("Main".to_string()))
        .insert_resource(LdtkSettings {
            int_grid_rendering: IntGridRendering::Invisible,
            ..default()
        })
        .add_plugin(LdtkPlugin)
        // .add_plugin(BlockPlugin)
        .add_system(bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    debug!("Spawning camera");
    commands.spawn_bundle(Camera2dBundle {
        projection: OrthographicProjection {
            window_origin: WindowOrigin::BottomLeft,
            ..default()
        },
        ..default()
    });

    commands.spawn_bundle(LdtkWorldBundle {
        ldtk_handle: asset_server.load("stotris.ldtk"),
        ..default()
    });
}
