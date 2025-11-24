use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;

use crate::combat;
use crate::combat::SpawnHitboxEvent;
use crate::game_state::AppState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                player_movement,
                player_attack,
                update_player_facing_direction,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::InGame), cleanup_game_entities);
    }
}

// -- Components --

#[derive(Component)]
pub struct Player {
    pub id: u8,
}

#[derive(Component)]
pub enum ControlType {
    Human,
    AI(BossType),
}

#[derive(Debug, Clone, Copy)]
pub enum BossType {
    NullPointer,
    UndefinedBehavior,
    DataRace,
    UseAfterFree,
    BufferOverflow,
}

#[derive(Component, Default)]
pub enum FacingDirection {
    #[default]
    Right,
    Left,
}

#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32,
}

#[derive(Component)]
pub struct MoveSpeed(pub f32);

// -- Systems --

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<(&mut LinearVelocity, &Player, &MoveSpeed, &ControlType, &Transform)>,
    player_transforms: Query<(&Transform, &Player, &ControlType)>,
) {
    let mut human_position = None;
    for (transform, player, control) in player_transforms.iter() {
        if matches!(control, ControlType::Human) {
            human_position = Some(transform.translation.x);
            break;
        }
    }

    for (mut velocity, player, move_speed, control, transform) in query.iter_mut() {
        let mut direction = 0.0;
        match control {
            ControlType::Human => {
                // Human player controls
                if keyboard_input.pressed(KeyCode::KeyA) {
                    direction -= 1.0;
                }
                if keyboard_input.pressed(KeyCode::KeyD) {
                    direction += 1.0;
                }
            }
            ControlType::AI(boss_type) => {
                // AI movement logic
                if let Some(human_x) = human_position {
                    let distance = human_x - transform.translation.x;
                    let abs_distance = distance.abs();

                    match boss_type {
                        BossType::NullPointer => {
                            // Vanishes occasionally - erratic movement
                            let random_time = time.elapsed_seconds() % 3.0;
                            if random_time > 2.5 {
                                direction = rand::random::<f32>() * 4.0 - 2.0; // Random direction
                            } else {
                                // Normal movement towards player
                                direction = if distance > 0.0 { 1.0 } else { -1.0 };
                            }
                        }
                        BossType::UndefinedBehavior => {
                            // Unpredictable erratic movement
                            let random_time = time.elapsed_seconds().sin() as f32;
                            direction = random_time * 2.0; // Sinusoidal erratic movement
                        }
                        BossType::DataRace => {
                            // Aggressive approach and retreat
                            let time_phase = (time.elapsed_seconds() * 2.0).sin() as f32;
                            direction = if time_phase > 0.0 { 1.0 } else { -1.0 };
                            if abs_distance < 100.0 {
                                direction *= -1.0; // Retreat when close
                            } else {
                                // Move towards player when far
                                direction = if distance > 0.0 { 1.0 } else { -1.0 };
                            }
                        }
                        BossType::UseAfterFree => {
                            // Steady aggressive approach
                            direction = if distance > 0.0 { 1.0 } else { -1.0 };
                        }
                        BossType::BufferOverflow => {
                            // Slow but steady towards player
                            direction = if distance > 0.0 { 0.5 } else { -0.5 };
                        }
                    }
                }
            }
        }
        velocity.x = direction * move_speed.0;
    }
}

fn player_attack(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    query: Query<(Entity, &Player, &ControlType, &Transform)>,
    player_transforms: Query<(&Transform, &ControlType)>,
    mut spawn_hitbox_writer: EventWriter<SpawnHitboxEvent>,
) {
    let mut human_position = None;
    for (transform, control) in player_transforms.iter() {
        if matches!(control, ControlType::Human) {
            human_position = Some(transform.translation);
            break;
        }
    }

    for (entity, player, control, transform) in query.iter() {
        let should_attack = match control {
            ControlType::Human => {
                // Human player controls
                keyboard_input.just_pressed(if player.id == 1 { KeyCode::KeyF } else { KeyCode::KeyL })
            }
            ControlType::AI(boss_type) => {
                // AI attack logic
                if let Some(human_pos) = human_position {
                    let distance = (human_pos - transform.translation).length();

                    match boss_type {
                        BossType::NullPointer => {
                            // Sporadic attacks
                            let phase = time.elapsed_seconds() % 4.0;
                            distance < 150.0 && phase > 3.5
                        }
                        BossType::UndefinedBehavior => {
                            // Random attacks
                            distance < 200.0 && rand::random::<f32>() < 0.05
                        }
                        BossType::DataRace => {
                            // Rapid attacks when close
                            distance < 120.0 && (time.elapsed_seconds() * 3.0).fract() < 0.3
                        }
                        BossType::UseAfterFree => {
                            // Steady attack intervals
                            distance < 180.0 && (time.elapsed_seconds() % 2.0) < 0.2
                        }
                        BossType::BufferOverflow => {
                            // Slow but powerful attacks
                            distance < 160.0 && (time.elapsed_seconds() % 5.0) < 0.5
                        }
                    }
                } else {
                    false
                }
            }
        };

        if should_attack {
            // Send one attack event per frame
            spawn_hitbox_writer.send(SpawnHitboxEvent { attacker: entity });
        }
    }
}

fn update_player_facing_direction(
    // Corrected: Removed `opponent_query` as it wasn't used
    mut player_query: Query<(&mut FacingDirection, &Transform, &Player)>,
) {
    let mut combinations = player_query.iter_combinations_mut();
    while let Some([(mut p1_dir, p1_t, _p1), (mut p2_dir, p2_t, _p2)]) = combinations.fetch_next() {
        if p1_t.translation.x < p2_t.translation.x {
            *p1_dir = FacingDirection::Right;
            *p2_dir = FacingDirection::Left;
        } else {
            *p1_dir = FacingDirection::Left;
            *p2_dir = FacingDirection::Right;
        }
    }
}

#[allow(dead_code)]
fn reset_players_on_restart(
    _commands: Commands,
    mut query: Query<(&mut Health, &mut Transform, &Player)>,
) {
    println!("Resetting player stats...");
    for (mut health, mut transform, player) in query.iter_mut() {
        health.current = health.max;
        if player.id == 1 {
            transform.translation = Vec3::new(-200.0, 0.0, 0.0);
        } else {
            transform.translation = Vec3::new(200.0, 0.0, 0.0);
        }
    }
}

#[allow(dead_code)]
pub fn setup_game_entities(mut commands: Commands) {
    println!("Setting up game entities...");

    // Some ground for the players to stand on
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.7, 0.7, 0.8),
                custom_size: Some(Vec2::new(1200.0, 50.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, -200.0, 0.0),
            ..default()
        },
        RigidBody::Static,
        Collider::rectangle(1200.0, 50.0),
    ));

    // -- Player 1 --
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_xyz(-200.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::rectangle(50.0, 100.0),
        combat::Hurtbox,
        Player { id: 1 },
        Health {
            current: 100,
            max: 100,
        },
        MoveSpeed(300.0),
        FacingDirection::Right,
    ));

    // -- Player 2 --
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::srgb(0.0, 0.0, 1.0),
                custom_size: Some(Vec2::new(50.0, 100.0)),
                ..default()
            },
            transform: Transform::from_xyz(200.0, 0.0, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        LockedAxes::ROTATION_LOCKED,
        Collider::rectangle(50.0, 100.0),
        combat::Hurtbox,
        Player { id: 2 },
        Health {
            current: 100,
            max: 100,
        },
        MoveSpeed(300.0),
        FacingDirection::Left,
    ));
}

type GameEntityQuery<'w, 's> = Query<'w, 's, Entity, Or<(With<Player>, With<RigidBody>)>>;

fn cleanup_game_entities(
    mut commands: Commands,
    query: GameEntityQuery,
) {
    println!("Cleaning up game entities...");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
