use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

use crate::{FacingDirection, GameState, GameplaySet, Money, MoneyEarnedEvent};

#[derive(Component, InspectorOptions, Default, Reflect)]
#[reflect(Component, InspectorOptions)]
pub struct Player {
    #[inspector(min = 0.0)]
    pub speed: f32,
    pub current_direction: FacingDirection,
    pub is_moving: bool,
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

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerAnimations {
            walk_down: vec![0, 4, 8, 12],
            walk_up: vec![2, 6, 10, 14],
            walk_left: vec![1, 5, 9, 13],
            walk_right: vec![3, 7, 11, 15],
        })
        .add_systems(Startup, spawn_player)
        .add_systems(
            Update,
            (character_movement, money_sound_effect, give_money)
                .in_set(GameplaySet::Player)
                .run_if(in_state(GameState::Gameplay)),
        )
        .add_systems(Update, (animate_player, animate_sprites).chain())
        .register_type::<Money>()
        .register_type::<Player>();
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
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
                is_moving: false,
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
        let movement_amount = player.speed * time.delta_seconds();

        let mut is_moving = false;

        if input.pressed(KeyCode::KeyW) {
            player.current_direction = FacingDirection::Up;
            transform.translation.y += movement_amount;
            is_moving = true;
        }
        if input.pressed(KeyCode::KeyS) {
            player.current_direction = FacingDirection::Down;
            transform.translation.y -= movement_amount;
            is_moving = true;
        }

        if input.pressed(KeyCode::KeyA) {
            player.current_direction = FacingDirection::Left;
            transform.translation.x -= movement_amount;
            is_moving = true;
        }
        if input.pressed(KeyCode::KeyD) {
            player.current_direction = FacingDirection::Right;
            transform.translation.x += movement_amount;
            is_moving = true;
        }

        player.is_moving = is_moving;

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

    if player.is_moving {
        match player.current_direction {
            FacingDirection::Up => {
                sprite.index =
                    animations.walk_up[animated_sprite.current_frame % animations.walk_up.len()];
            }
            FacingDirection::Down => {
                sprite.index = animations.walk_down
                    [animated_sprite.current_frame % animations.walk_down.len()];
            }
            FacingDirection::Left => {
                sprite.index = animations.walk_left
                    [animated_sprite.current_frame % animations.walk_left.len()];
            }
            FacingDirection::Right => {
                sprite.index = animations.walk_right
                    [animated_sprite.current_frame % animations.walk_right.len()];
            }
        }
    }
}
