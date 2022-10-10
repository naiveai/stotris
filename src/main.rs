mod block;

use bevy::{
    log::{Level, LogSettings},
    prelude::*,
    render::{
        camera::{ScalingMode, WindowOrigin},
        texture::ImageSettings,
    },
};
use bevy_ecs_ldtk::prelude::*;
use block::BlockPlugin;

const GAME_WIDTH: f32 = 512.0;
const GAME_HEIGHT: f32 = 640.0;

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
        width: GAME_WIDTH,
        height: GAME_HEIGHT,
        ..default()
    })
    .add_plugins(DefaultPlugins);

    #[cfg(feature = "inspector")]
    app.add_plugin(bevy_inspector_egui::WorldInspectorPlugin::new());

    app.add_startup_system(setup)
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(LevelSelection::Identifier("Main".to_string()))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: false,
            },
            int_grid_rendering: IntGridRendering::Invisible,
            set_clear_color: SetClearColor::FromLevelBackground,
            ..default()
        })
        .add_plugin(LdtkPlugin)
        .add_plugin(BlockPlugin)
        .add_system(camera_fit_inside_current_level)
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

const ASPECT_RATIO: f32 = GAME_WIDTH / GAME_HEIGHT;

pub fn camera_fit_inside_current_level(
    mut camera_query: Query<(
        &mut bevy::render::camera::OrthographicProjection,
        &mut Transform,
    )>,
    level_query: Query<(&Transform, &Handle<LdtkLevel>), Without<OrthographicProjection>>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

    for (level_transform, level_handle) in level_query.iter() {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level = &ldtk_level.level;
            let level_ratio = level.px_wid as f32 / ldtk_level.level.px_hei as f32;

            orthographic_projection.scaling_mode = ScalingMode::None;
            orthographic_projection.bottom = 0.;
            orthographic_projection.left = 0.;
            if level_ratio > ASPECT_RATIO {
                // level is wider than the screen
                orthographic_projection.top = (level.px_hei as f32 / 9.).round() * 9.;
                orthographic_projection.right = orthographic_projection.top * ASPECT_RATIO;
                camera_transform.translation.y = 0.;
            } else {
                // level is taller than the screen
                orthographic_projection.right = (level.px_wid as f32 / 16.).round() * 16.;
                orthographic_projection.top = orthographic_projection.right / ASPECT_RATIO;
                camera_transform.translation.x = 0.;
            }

            camera_transform.translation.x += level_transform.translation.x;
            camera_transform.translation.y += level_transform.translation.y;
        }
    }
}
