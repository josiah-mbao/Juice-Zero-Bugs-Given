use bevy::prelude::*;
use crate::game_state::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(Update, main_menu_interaction.run_if(in_state(AppState::MainMenu)))
            .add_systems(OnExit(AppState::MainMenu), cleanup_menu);
    }
}

// -- Components --

#[derive(Component)]
struct MenuButtonAction {
    action: MenuAction,
}

#[derive(Component)]
struct MainMenu;

#[derive(Debug, Clone, Copy)]
enum MenuAction {
    StartGame,
    Quit,
}

// -- Systems --

fn setup_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            background_color: Color::srgb(0.1, 0.1, 0.15).into(),
            ..default()
        },
        MainMenu,
    )).with_children(|parent| {
        // Title
        parent.spawn(TextBundle::from_section(
            "JUICE: ZERO BUGS GIVEN",
            TextStyle {
                font_size: 60.0,
                color: Color::WHITE,
                ..default()
            },
        ));

        // Subtitle
        parent.spawn(TextBundle::from_section(
            "Fight the bugs that Rust was designed to defeat!",
            TextStyle {
                font_size: 24.0,
                color: Color::srgb(0.8, 0.8, 0.8),
                ..default()
            },
        ));

        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(15.0),
                margin: UiRect::top(Val::Px(40.0)),
                ..default()
            },
            ..default()
        }).with_children(|parent| {
            // Start Game Button
            spawn_menu_button(parent, "START GAME", MenuAction::StartGame, &asset_server);
            
            // Quit Button
            spawn_menu_button(parent, "QUIT", MenuAction::Quit, &asset_server);
        });

        // Controls info
        parent.spawn(TextBundle::from_section(
            "Controls: Player 1 - A/D to move, F to attack | Player 2 - Arrows to move, L to attack",
            TextStyle {
                font_size: 16.0,
                color: Color::srgb(0.6, 0.6, 0.6),
                ..default()
            },
        ));
    });
}

fn spawn_menu_button(
    parent: &mut ChildBuilder,
    text: &str,
    action: MenuAction,
    asset_server: &Res<AssetServer>,
) {
    parent.spawn((
        ButtonBundle {
            style: Style {
                width: Val::Px(200.0),
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::srgb(0.15, 0.15, 0.2).into(),
            ..default()
        },
        MenuButtonAction { action },
    )).with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            text,
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        ));
    });
}

fn main_menu_interaction(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_state: ResMut<NextState<AppState>>,
) {
    for (interaction, button_action) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            match button_action.action {
                MenuAction::StartGame => {
                    println!("Starting game!");
                    app_state.set(AppState::InGame);
                }
                MenuAction::Quit => {
                    println!("Quitting game!");
                    std::process::exit(0);
                }
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MainMenu>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
