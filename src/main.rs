use bevy::log;
use bevy::prelude::*;
use board_plugin::resources::{BoardAssets, BoardOptions, SpriteMaterial};
use board_plugin::BoardPlugin;
use std::env;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Out,
    InPause,
}

fn main() {
    //  env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();
    // Window setup
    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper!".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    })
    // Board plugin options
    .add_startup_system(setup_board)
    .add_state(AppState::Out)
    .add_plugin(BoardPlugin {
        running_state: AppState::InGame,
    })
    // Bevy default plugins
    .add_plugins(DefaultPlugins)
    // Startup system (cameras)
    .add_startup_system(camera_setup);

    #[cfg(feature = "debug")]
    // Debug hierarchy inspector
    app.add_system(state_handler)
        .add_plugin(WorldInspectorPlugin::new())
        // Run the app
        .run();
}

fn state_handler(mut state: ResMut<State<AppState>>, keys: Res<Input<KeyCode>>) {
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if state.current() == &AppState::InGame {
            log::info!("clearing game");
            state.set(AppState::Out).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        if state.current() == &AppState::Out {
            log::info!("loading game");
            state.set(AppState::InGame).unwrap();
        } else {
            log::info!("reloading game");
            state.set(AppState::Out).unwrap();
            state.set(AppState::InGame).unwrap();
        }
    }
    if keys.just_pressed(KeyCode::Escape) {
        log::debug!("pauses detected");
        if state.current() == &AppState::InPause {
            log::info!("resuming game");
            state.set(AppState::InGame).unwrap();
        } else if state.current() == &AppState::InGame {
            log::info!("pausing game");
            state.set(AppState::InPause).unwrap();
        }
    }
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn setup_board(
    mut commands: Commands,
    mut state: ResMut<State<AppState>>,
    asset_server: Res<AssetServer>,
) {
    // Board plugin options
    commands.insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 1.,
        safe_start: true,
        ..Default::default()
    });
    // Board assets
    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial {
            color: Color::WHITE,
            ..Default::default()
        },
        tile_material: SpriteMaterial {
            color: Color::DARK_GRAY,
            ..Default::default()
        },
        covered_tile_material: SpriteMaterial {
            color: Color::GRAY,
            ..Default::default()
        },
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material: SpriteMaterial {
            texture: asset_server.load("sprites/flag.png"),
            color: Color::WHITE,
        },
        bomb_material: SpriteMaterial {
            texture: asset_server.load("sprites/bomb.png"),
            color: Color::WHITE,
        },
    });
    // Plugin activation
    state.set(AppState::InGame).unwrap();
}
