use bevy::{
    input::common_conditions::input_toggle_active, prelude::*, render::camera::ScalingMode,
};

use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use pig::PigPlugin;
use player::PlayerPlugin;
use rand::Rng;
use ui::GameUI;

mod pig;
mod player;
mod ui;

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

#[derive(Component, Reflect)]
pub enum FacingDirection {
    Up,
    Down,
    Left,
    Right,
}

// Implementing Default for Direction
impl Default for FacingDirection {
    fn default() -> Self {
        FacingDirection::Down
    }
}

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct Money(pub f32);

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
                        title: "Bevy RPG Game".into(),
                        resolution: (800.0, 600.0).into(),
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
        .add_event::<MoneyEarnedEvent>()
        .add_plugins(LdtkPlugin)
        .add_plugins((PlayerPlugin, PigPlugin, GameUI))
        .insert_state(GameState::Gameplay)
        .add_systems(Startup, setup)
        .insert_resource(LevelSelection::index(0))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Camera Setup
    let mut camera = Camera2dBundle::default();

    camera.projection.scale = 0.5;
    camera.transform.translation.x += 1280.0 / 4.0;
    camera.transform.translation.y += 720.0 / 4.0;

    // camera.projection.scaling_mode = ScalingMode::AutoMin {
    //     min_width: 256.0,
    //     min_height: 144.0,
    // };

    commands.spawn(camera);

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("tile-based-game.ldtk"),
        ..Default::default()
    });
}
