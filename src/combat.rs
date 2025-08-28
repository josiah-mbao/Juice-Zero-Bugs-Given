use bevy::prelude::*;
use bevy_xpbd_2d::prelude::*;
use std::time::Duration;

use crate::game_state::AppState;
use crate::player::{FacingDirection, Health, Player};

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnHitboxEvent>()
            .add_event::<DamageEvent>()
            .add_systems(
                Update,
                (
                    spawn_hitbox,
                    despawn_hitbox_after_duration,
                    detect_collisions,
                    apply_damage.after(detect_collisions),
                    check_for_game_over.after(apply_damage),
                )
                    .run_if(in_state(AppState::InGame)),
            );
    }
}

// -- Events --

#[derive(Event)]
pub struct SpawnHitboxEvent {
    pub attacker: Entity,
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

// -- Systems --

fn spawn_hitbox(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnHitboxEvent>,
    query: Query<&FacingDirection>,
) {
    for event in event_reader.read() {
        if let Ok(facing) = query.get(event.attacker) {
            let offset = match facing {
                FacingDirection::Right => Vec2::new(60.0, 0.0),
                FacingDirection::Left => Vec2::new(-60.0, 0.0),
            };

            commands
                .spawn((
                    SpatialBundle::from_transform(Transform::from_translation(offset.extend(0.0))),
                    // Corrected: Use .rectangle() instead of .cuboid()
                    Collider::rectangle(70.0, 40.0),
                    Sensor,
                    Hitbox {
                        damage: 10,
                        owner: event.attacker,
                    },
                    HitboxDuration {
                        timer: Timer::new(Duration::from_millis(150), TimerMode::Once),
                    },
                ))
                .set_parent(event.attacker);
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
    mut damage_writer: EventWriter<DamageEvent>,
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
                // Send one damage event
                damage_writer.send(DamageEvent {
                    target: hurtbox_entity,
                    damage: hitbox.damage,
                });

                // Added: Despawn the hitbox immediately so it can't hit again.
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
        if let Ok((mut health, player)) = query.get_mut(event.target) {
            health.current = (health.current - event.damage).max(0);
            println!("Player {} hit! {} HP left", player.id, health.current);
        }
    }
}

fn check_for_game_over(
    mut next_state: ResMut<NextState<AppState>>,
    query: Query<&Health, With<Player>>,
) {
    for health in query.iter() {
        if health.current <= 0 {
            println!("A player has been defeated! Game Over.");
            next_state.set(AppState::GameOver);
            break;
        }
    }
}
