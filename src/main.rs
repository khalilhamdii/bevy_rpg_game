use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

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
        .add_systems(Update, character_movement)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera Setup
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    // Player setup
    let texture = asset_server.load("character.png");

    commands
        .spawn((
            SpriteBundle {
                texture,
                ..Default::default()
            },
            Player { speed: 100.0 },
        ))
        .insert(Player { speed: 100.0 });
}

fn character_movement(
    mut characters: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut characters {
        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::KeyW) {
            transform.translation.y += movement_amount;
        }
        if input.pressed(KeyCode::KeyS) {
            transform.translation.y -= movement_amount;
        }
        if input.pressed(KeyCode::KeyD) {
            transform.translation.x += movement_amount;
        }
        if input.pressed(KeyCode::KeyA) {
            transform.translation.x -= movement_amount;
        }

        if input.pressed(KeyCode::KeyW) && input.pressed(KeyCode::KeyD) {
            transform.translation.x += movement_amount / 4.0;
            transform.translation.y += movement_amount / 4.0;
        }
        if input.pressed(KeyCode::KeyS) && input.pressed(KeyCode::KeyD) {
            transform.translation.x += movement_amount / 4.0;
            transform.translation.y -= movement_amount / 4.0;
        }
        if input.pressed(KeyCode::KeyW) && input.pressed(KeyCode::KeyA) {
            transform.translation.x -= movement_amount / 4.0;
            transform.translation.y += movement_amount / 4.0;
        }
        if input.pressed(KeyCode::KeyS) && input.pressed(KeyCode::KeyA) {
            transform.translation.x -= movement_amount / 4.0;
            transform.translation.y -= movement_amount / 4.0;
        }
    }
}
