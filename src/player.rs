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
    mut query: Query<(&mut LinearVelocity, &Player, &MoveSpeed)>,
) {
    for (mut velocity, player, move_speed) in query.iter_mut() {
        let mut direction = 0.0;
        if player.id == 1 {
            // Player 1 Controls (A, D)
            if keyboard_input.pressed(KeyCode::KeyA) {
                direction -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::KeyD) {
                direction += 1.0;
            }
        } else {
            // Player 2 Controls (Arrows)
            if keyboard_input.pressed(KeyCode::ArrowLeft) {
                direction -= 1.0;
            }
            if keyboard_input.pressed(KeyCode::ArrowRight) {
                direction += 1.0;
            }
        }
        velocity.x = direction * move_speed.0;
    }
}

fn player_attack(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<(Entity, &Player)>,
    mut spawn_hitbox_writer: EventWriter<SpawnHitboxEvent>,
) {
    for (entity, player) in query.iter() {
        let attack_pressed = if player.id == 1 {
            keyboard_input.just_pressed(KeyCode::KeyF)
        } else {
            keyboard_input.just_pressed(KeyCode::KeyL)
        };

        if attack_pressed {
            // Instead of handling logic here, we send an event.
            // This decouples the input from the combat system.
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

fn cleanup_game_entities(mut commands: Commands, query: GameEntityQuery) {
    println!("Cleaning up game entities...");
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
