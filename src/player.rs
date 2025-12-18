use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use std::time::Duration;

use crate::combat;
use crate::combat::SpawnHitboxEvent;
use crate::game_state::{AppState, BossType, Difficulty, GameConfig};
use crate::GameAssets;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                update_ai_state,
                player_movement.after(update_ai_state),
                player_jump.after(player_movement),
                play_jump_sound.after(player_jump),
                update_grounded.after(play_jump_sound),
                update_attack_cooldowns,
                player_attack.after(update_attack_cooldowns),
                player_block.after(update_attack_cooldowns),
                update_block_state.after(player_block),
                update_player_facing_direction,
            )
                .run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnExit(AppState::InGame), cleanup_game_entities);
    }
}

impl Difficulty {
    pub fn speed_multiplier(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.7,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 1.3,
        }
    }

    pub fn attack_frequency_multiplier(&self) -> f32 {
        match self {
            Difficulty::Easy => 1.5, // less frequent attacks
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 0.7, // more frequent attacks
        }
    }

    #[allow(dead_code)]
    pub fn health_multiplier(&self) -> f32 {
        match self {
            Difficulty::Easy => 0.8,
            Difficulty::Normal => 1.0,
            Difficulty::Hard => 1.2,
        }
    }
}

// -- Components --

#[derive(Component)]
pub struct Player {
    pub id: u8,
}

#[derive(Component, Clone)]
pub enum ControlType {
    Human,
    AI(BossType),
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

#[derive(Component)]
pub struct AttackCooldown {
    pub timer: Timer,
}

#[derive(Clone, Copy, PartialEq)]
pub enum AttackType {
    Light,
    Heavy,
    Kick,
}

#[derive(Component)]
pub struct BlockState {
    pub is_blocking: bool,
    pub block_timer: Timer,
    pub cooldown_timer: Timer,
}

#[derive(Component)]
pub struct Grounded(pub bool);

#[derive(Component, Clone, Default)]
pub enum AIState {
    #[default]
    Aggressive,
    Defensive,
    Erratic,
}

// -- Systems --

fn update_ai_state(time: Res<Time>, mut query: Query<(&Health, &mut AIState), With<ControlType>>) {
    for (health, mut state) in query.iter_mut() {
        let health_ratio = health.current as f32 / health.max as f32;

        // Change state based on health and time
        if health_ratio < 0.3 {
            *state = AIState::Defensive; // Low health: be defensive
        } else if (time.elapsed_seconds() as i32 % 10) < 3 {
            *state = AIState::Erratic; // Every 10 seconds, 3 seconds of erratic
        } else {
            *state = AIState::Aggressive; // Default aggressive
        }
    }
}

#[allow(clippy::type_complexity)]
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    config: Res<GameConfig>,
    mut query: Query<(
        &mut LinearVelocity,
        &Player,
        &MoveSpeed,
        &ControlType,
        &Transform,
        Option<&AIState>,
    )>,
    player_transforms: Query<(&Transform, &Player, &ControlType)>,
) {
    let mut human_position = None;
    for (transform, _player, control) in player_transforms.iter() {
        if matches!(control, ControlType::Human) {
            human_position = Some(transform.translation.x);
            break;
        }
    }

    for (mut velocity, _player, move_speed, control, transform, ai_state) in query.iter_mut() {
        let mut direction = 0.0;
        match control {
            ControlType::Human => {
                // Human player controls - different keys per player
                match _player.id {
                    1 => {
                        if keyboard_input.pressed(KeyCode::KeyA) {
                            direction -= 1.0;
                        }
                        if keyboard_input.pressed(KeyCode::KeyD) {
                            direction += 1.0;
                        }
                    }
                    2 => {
                        if keyboard_input.pressed(KeyCode::ArrowLeft) {
                            direction -= 1.0;
                        }
                        if keyboard_input.pressed(KeyCode::ArrowRight) {
                            direction += 1.0;
                        }
                    }
                    _ => {}
                }
            }
            ControlType::AI(boss_type) => {
                // AI movement logic
                if let Some(human_x) = human_position {
                    let distance = human_x - transform.translation.x;
                    let abs_distance = distance.abs();

                    let mut base_direction = match boss_type {
                        BossType::NullPointer => {
                            // Vanishes occasionally - erratic movement
                            let random_time = time.elapsed_seconds() % 3.0;
                            if random_time > 2.5 {
                                rand::random::<f32>() * 4.0 - 2.0 // Random direction
                            } else {
                                // Normal movement towards player
                                if distance > 0.0 {
                                    1.0
                                } else {
                                    -1.0
                                }
                            }
                        }
                        BossType::UndefinedBehavior => {
                            // Unpredictable erratic movement
                            time.elapsed_seconds().sin() * 2.0 // Sinusoidal erratic movement
                        }
                        BossType::DataRace => {
                            // Aggressive approach and retreat
                            let time_phase = (time.elapsed_seconds() * 2.0).sin();
                            let mut dir = if time_phase > 0.0 { 1.0 } else { -1.0 };
                            if abs_distance < 100.0 {
                                dir *= -1.0; // Retreat when close
                            } else {
                                // Move towards player when far
                                dir = if distance > 0.0 { 1.0 } else { -1.0 };
                            }
                            dir
                        }
                        BossType::UseAfterFree => {
                            // Steady aggressive approach
                            if distance > 0.0 {
                                1.0
                            } else {
                                -1.0
                            }
                        }
                        BossType::BufferOverflow => {
                            // Slow but steady towards player
                            if distance > 0.0 {
                                0.5
                            } else {
                                -0.5
                            }
                        }
                    };

                    // Apply AI state modifiers
                    if let Some(ai_state) = ai_state {
                        match ai_state {
                            AIState::Aggressive => {
                                // Boost towards player
                                base_direction *= 1.5;
                            }
                            AIState::Defensive => {
                                // Move away from player
                                base_direction = if distance > 0.0 { -1.0 } else { 1.0 };
                            }
                            AIState::Erratic => {
                                // Add randomness
                                base_direction += rand::random::<f32>() * 2.0 - 1.0;
                            }
                        }
                    }

                    direction = base_direction;
                }
            }
        }
        let speed_mult = if matches!(control, ControlType::AI(_)) {
            config.difficulty.speed_multiplier()
        } else {
            1.0
        };
        velocity.x = direction * move_speed.0 * speed_mult;
    }
}

fn update_attack_cooldowns(time: Res<Time>, mut query: Query<&mut AttackCooldown>) {
    for mut cooldown in query.iter_mut() {
        cooldown.timer.tick(time.delta());
    }
}

fn player_attack(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    config: Res<GameConfig>,
    mut query: Query<(
        Entity,
        &Player,
        &ControlType,
        &Transform,
        &mut AttackCooldown,
    )>,
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

    for (entity, player, control, transform, mut cooldown) in query.iter_mut() {
        // Only allow attacks if cooldown is finished
        if !cooldown.timer.finished() {
            continue;
        }

        // Determine attack type for human players
        let attack_type = match control {
            ControlType::Human => {
                let (light_key, heavy_key, kick_key) = if player.id == 1 {
                    (KeyCode::KeyF, KeyCode::KeyR, KeyCode::KeyT) // P1: F=Light, R=Heavy, T=Kick
                } else {
                    (KeyCode::KeyL, KeyCode::KeyO, KeyCode::KeyP) // P2: L=Light, O=Heavy, P=Kick
                };

                if keyboard_input.just_pressed(light_key) {
                    Some(AttackType::Light)
                } else if keyboard_input.just_pressed(heavy_key) {
                    Some(AttackType::Heavy)
                } else if keyboard_input.just_pressed(kick_key) {
                    Some(AttackType::Kick)
                } else {
                    None
                }
            }
            ControlType::AI(boss_type) => {
                // AI attack logic - determine attack type based on boss behavior
                if let Some(human_pos) = human_position {
                    let distance = (human_pos - transform.translation).length();

                    let should_attack = match boss_type {
                        BossType::NullPointer => {
                            // Sporadic attacks - prefers light attacks
                            let phase = time.elapsed_seconds() % 4.0;
                            let threshold = 3.5 * config.difficulty.attack_frequency_multiplier();
                            distance < 150.0 && phase > threshold
                        }
                        BossType::UndefinedBehavior => {
                            // Random attacks - mixed attack types
                            distance < 200.0 && rand::random::<f32>() < 0.05
                        }
                        BossType::DataRace => {
                            // Rapid attacks when close - prefers light attacks
                            distance < 120.0 && (time.elapsed_seconds() * 3.0).fract() < 0.3
                        }
                        BossType::UseAfterFree => {
                            // Steady attack intervals - prefers heavy attacks
                            distance < 180.0 && (time.elapsed_seconds() % 2.0) < 0.2
                        }
                        BossType::BufferOverflow => {
                            // Slow but powerful attacks - always heavy
                            distance < 160.0 && (time.elapsed_seconds() % 5.0) < 0.5
                        }
                    };

                    if should_attack {
                        // Choose attack type based on boss personality
                        Some(match boss_type {
                            BossType::NullPointer => AttackType::Light, // Fast erratic attacks
                            BossType::UndefinedBehavior => {
                                // Random attack type
                                match rand::random::<u32>() % 3 {
                                    0 => AttackType::Light,
                                    1 => AttackType::Heavy,
                                    _ => AttackType::Kick,
                                }
                            }
                            BossType::DataRace => AttackType::Light, // Fast rapid attacks
                            BossType::UseAfterFree => AttackType::Heavy, // Powerful steady attacks
                            BossType::BufferOverflow => AttackType::Heavy, // Slow powerful attacks
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        };

        if let Some(attack_type) = attack_type {
            // Start cooldown timer (longer for heavy attacks)
            let cooldown_duration = match attack_type {
                AttackType::Light => Duration::from_millis(250), // Quick recovery
                AttackType::Heavy => Duration::from_millis(400), // Longer recovery
                AttackType::Kick => Duration::from_millis(300),  // Medium recovery
            };
            cooldown.timer.set_duration(cooldown_duration);
            cooldown.timer.reset();

            // Send attack event with type
            spawn_hitbox_writer.send(SpawnHitboxEvent {
                attacker: entity,
                attack_type,
            });
        }
    }
}

fn player_block(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &ControlType, &mut BlockState)>,
) {
    for (player, control, mut block_state) in query.iter_mut() {
        if block_state.cooldown_timer.finished() {
            let should_block = match control {
                ControlType::Human => keyboard_input.just_pressed(match player.id {
                    1 => KeyCode::KeyS,
                    2 => KeyCode::ArrowDown,
                    _ => KeyCode::KeyS,
                }),
                ControlType::AI(_) => {
                    // AI blocks occasionally when health is low
                    false // For now, no AI blocking
                }
            };

            if should_block {
                block_state.is_blocking = true;
                block_state.block_timer.reset();
                block_state.cooldown_timer.reset();
                tracing::info!("Player {} started blocking", player.id);
            }
        }
    }
}

fn update_block_state(time: Res<Time>, mut query: Query<(&mut BlockState, &mut Sprite)>) {
    for (mut block_state, mut sprite) in query.iter_mut() {
        block_state.block_timer.tick(time.delta());
        block_state.cooldown_timer.tick(time.delta());

        if block_state.is_blocking && block_state.block_timer.finished() {
            block_state.is_blocking = false;
            tracing::info!("Block ended");
        }

        // Visual feedback: change color when blocking
        if block_state.is_blocking {
            sprite.color = Color::srgb(0.5, 0.5, 1.0); // Blue tint when blocking
        } else {
            // Reset to original color (this is simplistic - in reality you'd store original color)
            // For now, assume red for P1, blue for P2, but this will be overridden by boss colors
        }
    }
}

fn player_jump(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Player, &ControlType, &Grounded, &mut LinearVelocity)>,
) {
    for (player, control, grounded, mut velocity) in query.iter_mut() {
        if grounded.0 {
            let should_jump = match control {
                ControlType::Human => keyboard_input.just_pressed(match player.id {
                    1 => KeyCode::KeyW,
                    2 => KeyCode::ArrowUp,
                    _ => KeyCode::KeyW,
                }),
                ControlType::AI(_) => {
                    // AI jumps occasionally for variety
                    rand::random::<f32>() < 0.02 // 2% chance per frame when grounded
                }
            };

            if should_jump {
                velocity.y = 600.0; // Jump impulse
                tracing::info!("Player {} jumped", player.id);
            }
        }
    }
}

fn update_grounded(
    mut collision_events: EventReader<Collision>,
    mut query: Query<(Entity, &mut Grounded)>,
) {
    // First, set all players to not grounded
    for (_, mut grounded) in query.iter_mut() {
        grounded.0 = false;
    }

    // Then, set grounded to true if colliding with ground
    for collision in collision_events.read() {
        let contacts = &collision.0;
        for (entity, mut grounded) in query.iter_mut() {
            // Check if this entity is involved in the collision
            if contacts.entity1 == entity || contacts.entity2 == entity {
                // For simplicity, assume any collision means grounded
                // In a more complex system, you'd check if it's with the ground
                grounded.0 = true;
                break;
            }
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
    tracing::info!("Resetting player stats...");
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
    tracing::info!("Setting up game entities...");

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

fn cleanup_game_entities(mut commands: Commands, query: GameEntityQuery) {
    tracing::info!("Cleaning up game entities...");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn play_jump_sound(
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut event_reader: EventReader<crate::combat::SpawnHitboxEvent>,
) {
    // For now, we'll play jump sound when attacks happen (temporary)
    // In a real implementation, we'd have a separate JumpEvent
    for _ in event_reader.read() {
        commands.spawn(AudioBundle {
            source: assets.jump_sfx.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}
