use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use std::time::Duration;

use crate::game_state::{AppState, GameConfig, PlayerProgress, Winner};
use crate::player::{BlockState, ControlType, FacingDirection, Health, Player};
use crate::GameAssets;

#[derive(Resource)]
pub struct FightTracker {
    pub fight_start_time: Option<f32>,
    pub current_combo: u32,
    #[allow(dead_code)]
    pub boss: crate::game_state::BossType,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnHitboxEvent>()
            .add_event::<DamageEvent>()
            .add_systems(OnEnter(AppState::InGame), initialize_fight_tracker)
            .add_systems(
                Update,
                (
                    spawn_hitbox,
                    play_attack_sound.after(spawn_hitbox),
                    despawn_hitbox_after_duration,
                    detect_collisions,
                    play_hit_sound.after(detect_collisions),
                    apply_damage.after(detect_collisions),
                    spawn_particles_on_hit.after(apply_damage),
                    despawn_particles_after_duration.after(spawn_particles_on_hit),
                    check_for_game_over.after(apply_damage),
                    update_combo_tracker.after(apply_damage),
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

// -- Events --

#[derive(Event)]
pub struct SpawnHitboxEvent {
    pub attacker: Entity,
    pub attack_type: crate::player::AttackType,
}

#[derive(Event)]
pub struct DamageEvent {
    pub target: Entity,
    pub damage: i32,
}

// -- Components --

#[derive(Component)]
pub struct Hurtbox;

#[derive(Component)]
pub struct Hitbox {
    pub damage: i32,
    pub owner: Entity,
}

#[derive(Component)]
pub struct HitboxDuration {
    pub timer: Timer,
}

// -- Particle Components --

#[derive(Component)]
pub struct Particle;

#[derive(Component)]
pub struct ParticleDuration {
    pub timer: Timer,
}

// -- Systems --

fn spawn_hitbox(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnHitboxEvent>,
    query: Query<&FacingDirection>,
) {
    for event in event_reader.read() {
        match query.get(event.attacker) {
            Ok(facing) => {
                // Different properties based on attack type
                let (offset, size, damage, duration) = match event.attack_type {
                    crate::player::AttackType::Light => (
                        match facing {
                            FacingDirection::Right => Vec2::new(55.0, 0.0),
                            FacingDirection::Left => Vec2::new(-55.0, 0.0),
                        },
                        Vec2::new(60.0, 35.0),      // Smaller hitbox
                        3,                          // Lower damage
                        Duration::from_millis(120), // Shorter duration
                    ),
                    crate::player::AttackType::Heavy => (
                        match facing {
                            FacingDirection::Right => Vec2::new(70.0, 0.0),
                            FacingDirection::Left => Vec2::new(-70.0, 0.0),
                        },
                        Vec2::new(80.0, 45.0),      // Larger hitbox
                        8,                          // Higher damage
                        Duration::from_millis(200), // Longer duration
                    ),
                    crate::player::AttackType::Kick => (
                        match facing {
                            FacingDirection::Right => Vec2::new(65.0, -10.0), // Slightly lower for kick
                            FacingDirection::Left => Vec2::new(-65.0, -10.0),
                        },
                        Vec2::new(70.0, 40.0),      // Medium hitbox
                        5,                          // Medium damage
                        Duration::from_millis(160), // Medium duration
                    ),
                };

                commands
                    .spawn((
                        SpatialBundle::from_transform(Transform::from_translation(
                            offset.extend(0.0),
                        )),
                        Collider::rectangle(size.x, size.y),
                        Sensor,
                        Hitbox {
                            damage,
                            owner: event.attacker,
                        },
                        HitboxDuration {
                            timer: Timer::new(duration, TimerMode::Once),
                        },
                    ))
                    .set_parent(event.attacker);
            }
            Err(_) => {
                tracing::warn!(
                    "Attempted to spawn hitbox for entity without FacingDirection component"
                );
            }
        }
    }
}

fn despawn_hitbox_after_duration(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut HitboxDuration)>,
) {
    for (entity, mut duration) in query.iter_mut() {
        duration.timer.tick(time.delta());
        if duration.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn detect_collisions(
    mut commands: Commands, // Added: a way to issue commands like despawning
    mut collisions: EventReader<Collision>,
    hitbox_query: Query<&Hitbox>,
    hurtbox_query: Query<&Hurtbox>,
    block_query: Query<&BlockState>,
    mut damage_writer: EventWriter<DamageEvent>,
    transform_query: Query<&Transform>,
) {
    for Collision(contacts) in collisions.read() {
        // Determine which entity is the hitbox and which is the hurtbox
        let (hitbox_entity, hurtbox_entity) = if hitbox_query.contains(contacts.entity1)
            && hurtbox_query.contains(contacts.entity2)
        {
            (contacts.entity1, contacts.entity2)
        } else if hitbox_query.contains(contacts.entity2)
            && hurtbox_query.contains(contacts.entity1)
        {
            (contacts.entity2, contacts.entity1)
        } else {
            continue; // Not a hitbox-hurtbox collision
        };

        if let Ok(hitbox) = hitbox_query.get(hitbox_entity) {
            // Prevent hitting yourself
            if hitbox.owner != hurtbox_entity {
                // Check if target is blocking
                let is_blocking = block_query
                    .get(hurtbox_entity)
                    .map(|block_state| block_state.is_blocking)
                    .unwrap_or(false);

                if is_blocking {
                    // Blocked! No damage, reduced recoil
                    tracing::info!("Attack blocked!");

                    // Apply reduced recoil forces
                    if let (Ok(attacker_transform), Ok(defender_transform)) = (
                        transform_query.get(hitbox.owner),
                        transform_query.get(hurtbox_entity),
                    ) {
                        let direction_vec3 = (defender_transform.translation
                            - attacker_transform.translation)
                            .normalize();
                        let direction = Vec2::new(direction_vec3.x, direction_vec3.y);
                        let recoil_strength = 200.0; // Reduced recoil on block

                        // Push defender slightly
                        commands
                            .entity(hurtbox_entity)
                            .insert(ExternalImpulse::new(direction * recoil_strength * 0.5));

                        // Push attacker slightly backward
                        commands
                            .entity(hitbox.owner)
                            .insert(ExternalImpulse::new(-direction * recoil_strength * 0.2));
                    }
                } else {
                    // Normal hit - send damage event
                    damage_writer.send(DamageEvent {
                        target: hurtbox_entity,
                        damage: hitbox.damage,
                    });

                    // Apply full recoil forces
                    if let (Ok(attacker_transform), Ok(defender_transform)) = (
                        transform_query.get(hitbox.owner),
                        transform_query.get(hurtbox_entity),
                    ) {
                        let direction_vec3 = (defender_transform.translation
                            - attacker_transform.translation)
                            .normalize();
                        let direction = Vec2::new(direction_vec3.x, direction_vec3.y);
                        let recoil_strength = 500.0; // Force to push players apart

                        // Push defender away from attacker
                        commands
                            .entity(hurtbox_entity)
                            .insert(ExternalImpulse::new(direction * recoil_strength));

                        // Push attacker slightly backward
                        commands
                            .entity(hitbox.owner)
                            .insert(ExternalImpulse::new(-direction * recoil_strength * 0.3));
                    }
                }

                // Despawn the hitbox immediately so it can't hit again.
                commands.entity(hitbox_entity).despawn_recursive();
            }
        }
    }
}

fn apply_damage(
    // Corrected: Removed `mut commands: Commands` as it wasn't used
    mut damage_reader: EventReader<DamageEvent>,
    mut query: Query<(&mut Health, &Player)>,
) {
    for event in damage_reader.read() {
        match query.get_mut(event.target) {
            Ok((mut health, player)) => {
                health.current = (health.current - event.damage).max(0);
                tracing::info!("Player {} hit! {} HP left", player.id, health.current);
            }
            Err(_) => {
                tracing::warn!(
                    "Tried to apply damage to entity without Health or Player components"
                );
            }
        }
    }
}

fn spawn_particles_on_hit(
    mut commands: Commands,
    mut damage_reader: EventReader<DamageEvent>,
    transform_query: Query<&Transform>,
) {
    for event in damage_reader.read() {
        if let Ok(transform) = transform_query.get(event.target) {
            let position = transform.translation.truncate();
            let particle_count = 5;
            for i in 0..particle_count {
                let angle = std::f32::consts::PI * 2.0 * (i as f32 / particle_count as f32);
                let speed = 100.0 + rand::random::<f32>() * 50.0;
                let velocity = Vec2::new(angle.cos(), angle.sin()) * speed;

                commands.spawn((
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::srgb(1.0, 0.0, 0.0),
                            custom_size: Some(Vec2::new(5.0, 5.0)),
                            ..default()
                        },
                        transform: Transform::from_xyz(position.x, position.y, 1.0),
                        ..default()
                    },
                    RigidBody::Dynamic,
                    LinearVelocity(velocity),
                    Particle,
                    ParticleDuration {
                        timer: Timer::new(Duration::from_millis(500), TimerMode::Once),
                    },
                ));
            }
        }
    }
}

fn despawn_particles_after_duration(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ParticleDuration), With<Particle>>,
) {
    for (entity, mut duration) in query.iter_mut() {
        duration.timer.tick(time.delta());
        if duration.timer.finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn check_for_game_over(
    mut next_state: ResMut<NextState<AppState>>,
    query: Query<(&Health, &Player, &ControlType)>,
    mut winner: ResMut<Winner>,
    config: Res<GameConfig>,
    mut progress: ResMut<PlayerProgress>,
    fight_tracker: Res<FightTracker>,
    time: Res<Time>,
) {
    let mut players_alive = Vec::new();
    let mut players_dead = Vec::new();

    for (health, player, control) in query.iter() {
        if health.current <= 0 {
            players_dead.push((player.id, control));
        } else {
            players_alive.push((player.id, control));
        }
    }

    if players_alive.len() == 1 {
        let (winner_id, winner_control) = players_alive[0];
        winner.player_id = Some(winner_id);
        winner.is_human_winner = Some(matches!(winner_control, ControlType::Human));

        // Record victory/defeat statistics
        let fight_duration = if let Some(start_time) = fight_tracker.fight_start_time {
            time.elapsed_seconds() - start_time
        } else {
            0.0
        };

        if matches!(winner_control, ControlType::Human) && !config.player2_is_human {
            // Human victory
            progress.record_victory(config.boss, fight_duration, fight_tracker.current_combo);

            // Check if this was the final boss
            if config.boss.is_final_boss() {
                tracing::info!("All bosses defeated! Showing credits.");
                next_state.set(AppState::Credits);
            } else {
                // Unlock next boss if human player won against AI
                if let Some(next_boss) = progress.get_next_boss(config.boss) {
                    progress.unlock_boss(next_boss);
                    tracing::info!("New boss unlocked: {:?}", next_boss);
                }
                tracing::info!("Player {} wins! Game Over.", winner_id);
                next_state.set(AppState::GameOver);
            }
        } else {
            // AI victory or human vs human
            progress.record_defeat(config.boss);
            tracing::info!("Player {} wins! Game Over.", winner_id);
            next_state.set(AppState::GameOver);
        }
    } else if players_alive.is_empty() && !config.player2_is_human {
        // Both died, but only if vs AI (since human vs human doesn't make sense for draw)
        winner.player_id = None;
        winner.is_human_winner = None;

        // Record defeat for both (since it's a draw)
        progress.record_defeat(config.boss);

        tracing::info!("Both players died! Draw.");
        next_state.set(AppState::GameOver);
    }
}

fn play_attack_sound(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnHitboxEvent>,
    assets: Res<GameAssets>,
) {
    for _event in event_reader.read() {
        // Play attack sound
        commands.spawn(AudioBundle {
            source: assets.attack_sfx.clone(),
            settings: PlaybackSettings::DESPAWN,
        });
    }
}

fn play_hit_sound(
    mut commands: Commands,
    mut event_reader: EventReader<DamageEvent>,
    block_query: Query<&BlockState>,
    assets: Res<GameAssets>,
) {
    for event in event_reader.read() {
        // Check if the target was blocking
        let is_blocking = block_query
            .get(event.target)
            .map(|block_state| block_state.is_blocking)
            .unwrap_or(false);

        if is_blocking {
            // Play block sound
            commands.spawn(AudioBundle {
                source: assets.block_sfx.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
        } else {
            // Play hit sound
            commands.spawn(AudioBundle {
                source: assets.hit_sfx.clone(),
                settings: PlaybackSettings::DESPAWN,
            });
        }
    }
}

fn initialize_fight_tracker(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut progress: ResMut<PlayerProgress>,
    time: Res<Time>,
) {
    // Record fight start
    progress.record_fight_start(config.boss);

    commands.insert_resource(FightTracker {
        fight_start_time: Some(time.elapsed_seconds()),
        current_combo: 0,
        boss: config.boss,
    });
}

fn update_combo_tracker(
    mut fight_tracker: ResMut<FightTracker>,
    mut damage_events: EventReader<DamageEvent>,
    block_query: Query<&BlockState>,
    _time: Res<Time>,
) {
    let mut hits_this_frame = 0;

    // Count successful hits (not blocked)
    for event in damage_events.read() {
        let is_blocking = block_query
            .get(event.target)
            .map(|block_state| block_state.is_blocking)
            .unwrap_or(false);

        if !is_blocking {
            hits_this_frame += 1;
        }
    }

    if hits_this_frame > 0 {
        fight_tracker.current_combo += hits_this_frame;
        tracing::info!("Combo: {}x", fight_tracker.current_combo);
    }
}
