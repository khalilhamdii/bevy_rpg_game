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
    pub current_direction: FacingDirection,
    pub moving: bool,
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
pub struct PlayerAnimations {
    pub walk_down: Vec<usize>,
    pub walk_up: Vec<usize>,
    pub walk_left: Vec<usize>,
    pub walk_right: Vec<usize>,
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AnimatedSprite {
    pub current_frame: usize,
    pub timer: Timer,
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
        .insert_resource(PlayerAnimations {
            walk_down: vec![0, 4, 8, 12],
            walk_up: vec![2, 6, 10, 14],
            walk_left: vec![1, 5, 9, 13],
            walk_right: vec![3, 7, 11, 15],
        })
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
        .add_systems(Update, (animate_player, animate_sprites).chain())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Camera Setup
    let mut camera = Camera2dBundle::default();

    camera.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: 256.0,
        min_height: 144.0,
    };

    commands.spawn(camera);

    // Player setup

    // Load the texture for the sprite sheet
    let texture_handle = asset_server.load("player_spritesheet.png");

    // Create a TextureAtlas from the sprite sheet (assuming it's a 4x4 grid)
    let texture_layout = TextureAtlasLayout::from_grid(UVec2::splat(48), 4, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_layout);

    // Spawn the player with the sprite sheet and animation
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_scale(Vec3::splat(0.5)),
                texture: texture_handle.clone(),
                ..Default::default()
            },
            Player {
                speed: 100.0,
                moving: false,
                ..Default::default()
            },
            Name::new("Player"),
            TextureAtlas {
                layout: texture_atlas_handle.clone(),
                index: 0,
            },
        ))
        .insert(AnimatedSprite {
            current_frame: 0,
            timer: Timer::from_seconds(0.2, TimerMode::Repeating),
        });
}

fn character_movement(
    mut player_query: Query<(&mut Transform, &mut Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, mut player) in &mut player_query {
        player.moving = false;

        let movement_amount = player.speed * time.delta_seconds();

        if input.pressed(KeyCode::KeyW) {
            player.current_direction = FacingDirection::Up;
            transform.translation.y += movement_amount;
            player.moving = true;
        }
        if input.pressed(KeyCode::KeyS) {
            player.current_direction = FacingDirection::Down;
            transform.translation.y -= movement_amount;
            player.moving = true;
        }

        if input.pressed(KeyCode::KeyA) {
            player.current_direction = FacingDirection::Left;
            transform.translation.x -= movement_amount;
            player.moving = true;
        }
        if input.pressed(KeyCode::KeyD) {
            player.current_direction = FacingDirection::Right;
            transform.translation.x += movement_amount;
            player.moving = true;
        }

        // if input.pressed(KeyCode::KeyW) && input.pressed(KeyCode::KeyD) {
        //     transform.translation.x += movement_amount / 4.0;
        //     transform.translation.y += movement_amount / 4.0;
        // }
        // if input.pressed(KeyCode::KeyS) && input.pressed(KeyCode::KeyD) {
        //     transform.translation.x += movement_amount / 4.0;
        //     transform.translation.y -= movement_amount / 4.0;
        // }
        // if input.pressed(KeyCode::KeyW) && input.pressed(KeyCode::KeyA) {
        //     transform.translation.x -= movement_amount / 4.0;
        //     transform.translation.y += movement_amount / 4.0;
        // }
        // if input.pressed(KeyCode::KeyS) && input.pressed(KeyCode::KeyA) {
        //     transform.translation.x -= movement_amount / 4.0;
        //     transform.translation.y -= movement_amount / 4.0;
        // }
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

fn animate_sprites(mut sprites: Query<&mut AnimatedSprite>, time: Res<Time>) {
    for mut sprite in sprites.iter_mut() {
        sprite.timer.tick(time.delta());
        if sprite.timer.just_finished() {
            sprite.current_frame += 1;
        }
    }
}

fn animate_player(
    mut player_query: Query<(&mut TextureAtlas, &AnimatedSprite, &Player)>,
    animations: Res<PlayerAnimations>,
) {
    let (mut sprite, animated_sprite, player) = player_query.single_mut();

    match player.current_direction {
        FacingDirection::Up => {
            sprite.index =
                animations.walk_up[animated_sprite.current_frame % animations.walk_up.len()];
        }
        FacingDirection::Down => {
            sprite.index =
                animations.walk_down[animated_sprite.current_frame % animations.walk_down.len()];
        }
        FacingDirection::Left => {
            sprite.index =
                animations.walk_left[animated_sprite.current_frame % animations.walk_left.len()];
        }
        FacingDirection::Right => {
            sprite.index =
                animations.walk_right[animated_sprite.current_frame % animations.walk_right.len()];
        }
    }
}
