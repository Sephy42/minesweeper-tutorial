use bevy::log;
use bevy::prelude::*;
use board_plugin::resources::BoardOptions;
use board_plugin::BoardPlugin;
use std::env;

#[cfg(feature = "debug")]
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    InGame,
    Out,
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut app = App::new();
    // Window setup
    app.insert_resource(WindowDescriptor {
        title: "Mine Sweeper!".to_string(),
        width: 700.,
        height: 800.,
        ..Default::default()
    })
    // Board plugin options
    .insert_resource(BoardOptions {
        map_size: (20, 20),
        bomb_count: 40,
        tile_padding: 3.0,
        safe_start: true,
        ..Default::default()
    })
    .add_state(AppState::InGame)
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
        }
    }
}

fn camera_setup(mut commands: Commands) {
    // 2D orthographic camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
