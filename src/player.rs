use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;
use bevy_inspector_egui::prelude::*;
use std::collections::HashSet;

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

// const GRID_SIZE: i32 = 16;

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
        .add_systems(Update, (animate_player, animate_sprites, move_camera))
        .register_type::<Money>()
        .register_type::<Player>();
        // .insert_resource(LevelSelection::index(0))
        // .register_ldtk_int_cell::<WallBundle>(1)
        // .init_resource::<LevelWalls>()
        // .register_ldtk_entity::<PlayerBundle>("Player")
        // .register_ldtk_entity::<GoalBundle>("Goal")
        // .add_systems(
        //     Update,
        //     (
        //         move_player_from_input,
        //         translate_grid_coords_entities,
        //         cache_wall_locations,
        //         check_goal,
        //         move_camera,
        //     ),
        // );
    }
}

// fn move_player_from_input(
//     // mut player_query: Query<(&mut Transform, &mut Player)>,
//     mut player_query: Query<(&mut GridCoords, &mut Player)>,
//     input: Res<ButtonInput<KeyCode>>,
//     level_walls: Res<LevelWalls>,
//     time: Res<Time>,
// ) {
//     for (mut player_grid_coords, mut player) in &mut player_query {
//         let movement_amount = (100.0 * time.delta_seconds()).round() as i32;
//         let mut movement_direction = GridCoords::new(0, 0);

//         let mut is_moving = false;

//         if input.pressed(KeyCode::KeyW) {
//             player.current_direction = FacingDirection::Up;
//             movement_direction = GridCoords::new(0, movement_amount);
//             is_moving = true;
//         }
//         if input.pressed(KeyCode::KeyS) {
//             player.current_direction = FacingDirection::Down;
//             movement_direction = GridCoords::new(0, -movement_amount);
//             is_moving = true;
//         }

//         if input.pressed(KeyCode::KeyA) {
//             player.current_direction = FacingDirection::Left;
//             movement_direction = GridCoords::new(-movement_amount, 0);
//             is_moving = true;
//         }
//         if input.pressed(KeyCode::KeyD) {
//             player.current_direction = FacingDirection::Right;
//             movement_direction = GridCoords::new(movement_amount, 0);
//             is_moving = true;
//         }

//         let destination = *player_grid_coords + movement_direction;
//         if !level_walls.in_wall(&destination) {
//             is_moving = true;
//             *player_grid_coords = destination;
//         }

//         player.is_moving = is_moving;
//     }
// }

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
    }
}

// fn translate_grid_coords_entities(
//     mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
// ) {
//     for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
//         transform.translation =
//             bevy_ecs_ldtk::utils::grid_coords_to_translation(*grid_coords, IVec2::splat(GRID_SIZE))
//                 .extend(transform.translation.z);
//     }
// }

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
                transform: Transform::from_xyz(47.0, 59.0, 1.5).with_scale(Vec3::splat(0.5)),
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

pub fn move_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation.x = player_transform.translation.x;
        camera_transform.translation.y = player_transform.translation.y;
    }
}

// fn cache_wall_locations(
//     mut level_walls: ResMut<LevelWalls>,
//     mut level_events: EventReader<LevelEvent>,
//     walls: Query<&GridCoords, With<Wall>>,
//     ldtk_project_entities: Query<&Handle<LdtkProject>>,
//     ldtk_project_assets: Res<Assets<LdtkProject>>,
// ) {
//     for level_event in level_events.read() {
//         if let LevelEvent::Spawned(level_iid) = level_event {
//             let ldtk_project = ldtk_project_assets
//                 .get(ldtk_project_entities.single())
//                 .expect("LdtkProject should be loaded when level is spawned");
//             let level = ldtk_project
//                 .get_raw_level_by_iid(level_iid.get())
//                 .expect("spawned level should exist in project");

//             let wall_locations = walls.iter().copied().collect();

//             let new_level_walls = LevelWalls {
//                 wall_locations,
//                 level_width: level.px_wid / GRID_SIZE,
//                 level_height: level.px_hei / GRID_SIZE,
//             };

//             *level_walls = new_level_walls;
//         }
//     }
// }

// fn check_goal(
//     level_selection: ResMut<LevelSelection>,
//     players: Query<&GridCoords, (With<Player>, Changed<GridCoords>)>,
//     goals: Query<&GridCoords, With<Goal>>,
// ) {
//     if players
//         .iter()
//         .zip(goals.iter())
//         .any(|(player_grid_coords, goal_grid_coords)| player_grid_coords == goal_grid_coords)
//     {
//         let indices = match level_selection.into_inner() {
//             LevelSelection::Indices(indices) => indices,
//             _ => panic!("level selection should always be Indices in this game"),
//         };

//         // info!("The indices level is: {:#?}", indices.level);

//         if indices.level == 0 {
//             indices.level += 1;
//         } else {
//             indices.level -= 1;
//         }
//     }
// }

// #[derive(Default, Bundle, LdtkEntity)]
// struct PlayerBundle {
//     player: Player,
//     #[sprite_sheet_bundle]
//     sprite_sheet_bundle: LdtkSpriteSheetBundle,
//     #[grid_coords]
//     grid_coords: GridCoords,
// }

// #[derive(Default, Component)]
// struct Goal;

// #[derive(Default, Bundle, LdtkEntity)]
// struct GoalBundle {
//     goal: Goal,
//     #[sprite_sheet_bundle]
//     sprite_bundle: LdtkSpriteSheetBundle,
//     #[grid_coords]
//     grid_coords: GridCoords,
// }

// #[derive(Default, Component)]
// struct Wall;

// #[derive(Default, Bundle, LdtkIntCell)]
// struct WallBundle {
//     wall: Wall,
// }

// #[derive(Default, Resource)]
// struct LevelWalls {
//     wall_locations: HashSet<GridCoords>,
//     level_width: i32,
//     level_height: i32,
// }

// impl LevelWalls {
//     fn in_wall(&self, grid_coords: &GridCoords) -> bool {
//         grid_coords.x < 0
//             || grid_coords.y < 0
//             || grid_coords.x >= self.level_width
//             || grid_coords.y >= self.level_height
//             || self.wall_locations.contains(grid_coords)
//     }
// }
