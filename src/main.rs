use bevy::{prelude::*, render::camera::ScalingMode};
use rand::seq::SliceRandom;
#[derive(Component)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct Pig {
    pub lifetime: Timer,
    pub speed: f32,
    pub current_direction: Direction,
    pub direction_timer: Timer, // Timer to change direction
}

#[derive(Resource)]
pub struct Money(pub f32);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
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
        .insert_resource(Money(100.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (character_movement, spawn_pig, pig_lifetime))
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

fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 on a pig, remaining money: ${:?}", money.0);

        let texture = asset_server.load("pig.png");

        commands.spawn((
            SpriteBundle {
                texture,
                transform: *player_transform,
                ..Default::default()
            },
            Pig {
                lifetime: Timer::from_seconds(10.0, TimerMode::Once),
                speed: 25.0,
                current_direction: Direction::Up,
                direction_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            },
        ));
    }
}

fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig, &mut Transform)>,
    mut money: ResMut<Money>,
) {
    let mut rng = rand::thread_rng();
    let directions = vec![
        Direction::Up,
        Direction::Down,
        Direction::Right,
        Direction::Left,
    ];
    for (pig_entity, mut pig, mut transform) in &mut pigs {
        pig.lifetime.tick(time.delta());
        pig.direction_timer.tick(time.delta());

        if pig.direction_timer.finished() {
            pig.current_direction = *directions.choose(&mut rng).unwrap();
        }

        if pig.lifetime.finished() {
            money.0 += 15.0;
            commands.entity(pig_entity).despawn();

            info!("Pig sold for $15! Current Money: ${:?}", money.0)
        }

        let movement_amount = pig.speed * time.delta_seconds(); // Example movement speed
        match pig.current_direction {
            Direction::Up => transform.translation.y += movement_amount,
            Direction::Down => transform.translation.y -= movement_amount,
            Direction::Right => transform.translation.x += movement_amount,
            Direction::Left => transform.translation.x -= movement_amount,
        }
    }
}
