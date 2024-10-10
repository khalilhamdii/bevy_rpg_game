use bevy::prelude::*;
use rand::seq::SliceRandom;

use crate::{Direction, GameState, GameplaySet, Money, MoneyEarnedEvent, Player};

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Pig {
    pub lifetime: Timer,
    pub speed: f32,
    pub current_direction: Direction,
    pub direction_timer: Timer, // Timer to change direction
}

#[derive(Component)]
pub struct PigParent;

pub struct PigPlugin;

impl Plugin for PigPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_pig_parent)
            .add_systems(
                Update,
                (spawn_pig, pig_lifetime)
                    .in_set(GameplaySet::Pig)
                    .run_if(in_state(GameState::Gameplay)),
            )
            .register_type::<Pig>();
    }
}

fn spawn_pig(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut money: ResMut<Money>,
    player: Query<&Transform, With<Player>>,
    parent: Query<Entity, With<PigParent>>,
) {
    if !input.just_pressed(KeyCode::Space) {
        return;
    }

    let player_transform = player.single();
    let parent = parent.single();

    if money.0 >= 10.0 {
        money.0 -= 10.0;
        info!("Spent $10 on a pig, remaining money: ${:?}", money.0);

        let texture = asset_server.load("pig.png");

        // commands.entity(parent).with_children(|commands| {});

        commands
            .spawn((
                SpriteBundle {
                    texture,
                    transform: *player_transform,
                    ..Default::default()
                },
                Pig {
                    lifetime: Timer::from_seconds(10.0, TimerMode::Once),
                    speed: 25.0,
                    current_direction: Direction::default(),
                    direction_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
                },
                Name::new("Pig"),
            ))
            .set_parent(parent);
    }
}

fn pig_lifetime(
    mut commands: Commands,
    time: Res<Time>,
    mut pigs: Query<(Entity, &mut Pig, &mut Transform)>,
    mut event_writer: EventWriter<MoneyEarnedEvent>,
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
            event_writer.send(MoneyEarnedEvent(15.0));
            commands.entity(pig_entity).despawn_recursive();
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

fn spawn_pig_parent(mut commands: Commands) {
    commands.spawn((SpatialBundle::default(), PigParent, Name::new("Pig Parent")));
}
