use bevy::prelude::*;

use crate::game_state::AppState;
use crate::player::{Health, Player};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_ui)
            .add_systems(Update, update_health_bars.run_if(in_state(AppState::InGame)))
            .add_systems(OnEnter(AppState::GameOver), setup_game_over_screen)
            .add_systems(OnExit(AppState::GameOver), cleanup_game_over_screen);
    }
}

// -- Components (for tagging UI elements) --

#[derive(Component)]
struct HealthBar(u8); // Holds the player ID (1 or 2)

#[derive(Component)]
struct GameOverScreen;

// -- Systems --

fn setup_ui(mut commands: Commands) {
    // Player 1 Health Bar
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(40.0),
            height: Val::Px(30.0),
            left: Val::Percent(5.0),
            top: Val::Percent(5.0),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        border_color: Color::WHITE.into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                // Corrected: Use a specific srgb color for green.
                background_color: Color::srgb(0.1, 0.9, 0.1).into(),
                ..default()
            },
            HealthBar(1),
        ));
    });

    // Player 2 Health Bar
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            width: Val::Percent(40.0),
            height: Val::Px(30.0),
            right: Val::Percent(5.0),
            top: Val::Percent(5.0),
            border: UiRect::all(Val::Px(2.0)),
            ..default()
        },
        border_color: Color::WHITE.into(),
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                // Corrected: Use a specific srgb color for green.
                background_color: Color::srgb(0.1, 0.9, 0.1).into(),
                ..default()
            },
            HealthBar(2),
        ));
    });
}

fn update_health_bars(
    player_query: Query<(&Health, &Player)>,
    mut health_bar_query: Query<(&mut Style, &HealthBar)>,
) {
    for (mut style, health_bar) in health_bar_query.iter_mut() {
        for (health, player) in player_query.iter() {
            if player.id == health_bar.0 {
                style.width = Val::Percent((health.current as f32 / health.max as f32) * 100.0);
            }
        }
    }
}

fn setup_game_over_screen(mut commands: Commands) {
    commands.spawn((
        NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                row_gap: Val::Px(20.0),
                ..default()
            },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
            ..default()
        },
        GameOverScreen,
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "GAME OVER",
            TextStyle {
                font_size: 100.0,
                color: Color::WHITE,
                ..default()
            },
        ));
        parent.spawn(TextBundle::from_section(
            "Press SPACE to Restart",
            TextStyle {
                font_size: 40.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

fn cleanup_game_over_screen(
    mut commands: Commands,
    query: Query<Entity, With<GameOverScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
