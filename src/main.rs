use bevy::{core_pipeline::core_2d::graph::Core2d, prelude::*};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Farming Rougelike".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .build(),
        )
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn Camera
    commands.spawn(Camera2dBundle {
        camera: Camera {
            clear_color: ClearColorConfig::Custom(Color::srgb(0.04, 0.0, 0.07)),
            ..Default::default()
        },
        ..Default::default()
    });

    let texture = asset_server.load("character.png");

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..Default::default()
        },
        texture,
        ..Default::default()
    });
}
