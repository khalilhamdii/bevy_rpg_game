use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode,
};

use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use pig::PigPlugin;
use rand::Rng;
use ui::GameUI;

mod pig;
mod ui;

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)]
    pub speed: f32,
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Money(pub f32);

#[derive(Debug, Clone, Copy, Reflect)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

// Implementing Default for Direction
impl Default for Direction {
    fn default() -> Self {
        let mut rng = rand::thread_rng();
        match rng.gen_range(0..4) {
            0 => Direction::Up,
            1 => Direction::Down,
            2 => Direction::Right,
            _ => Direction::Left,
        }
    }
}

#[derive(Event)]
pub struct MoneyEarnedEvent(f32);

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    MainMenu,
    Gameplay,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameplaySet {
    Player,
    Pig,
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
        .add_plugins(
            WorldInspectorPlugin::default().run_if(input_toggle_active(true, KeyCode::Escape)),
        )
        .insert_resource(Money(100.0))
        .register_type::<Money>()
        .register_type::<Player>()
        .add_event::<MoneyEarnedEvent>()
        .add_plugins((PigPlugin, GameUI))
        .insert_state(GameState::Gameplay)
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (character_movement, money_sound_effect, give_money)
                .in_set(GameplaySet::Player)
                .run_if(in_state(GameState::Gameplay)),
        )
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
            Name::new("Player"),
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

fn money_sound_effect(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut money_events: EventReader<MoneyEarnedEvent>,
) {
    for _event in money_events.read() {
        commands.spawn((
            AudioBundle {
                source: assets.load("money.wav"),
                ..Default::default()
            },
            Name::new("MoneyAudio"),
        ));
    }
}

fn give_money(mut money_events: EventReader<MoneyEarnedEvent>, mut money: ResMut<Money>) {
    for event in money_events.read() {
        money.0 += event.0
    }
}
