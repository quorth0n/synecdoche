use std::time::Duration;

use crate::components::Province;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

mod components;

const TICK_LENGTH: u64 = 200;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TickTimer(Timer::from_seconds(TICK_LENGTH as f32, true)))
        .insert_resource(GameSpeed(0))
        .add_startup_system(setup_system)
        .add_state(GameState::Paused)
        // tick systems
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(greet_people))
        // UI
        .add_plugin(EguiPlugin)
        .add_system(paint_ui)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Paused,
    Playing,
}

struct GameSpeed(u64);

struct TickTimer(Timer);

fn greet_people(time: Res<Time>, mut timer: ResMut<TickTimer>, mut query: Query<&mut Province>) {
    if timer.0.tick(time.delta()).just_finished() {
        for mut city in query.iter_mut() {
            city.pops += 100;
            println!("hello {}!", city.pops);
        }
    }
}

fn setup_system(mut commands: Commands) {
    commands.spawn().insert(Province { pops: 10000 });
}

fn paint_ui(
    mut egui_context: ResMut<EguiContext>,
    mut speed: ResMut<GameSpeed>,
    mut timer: ResMut<TickTimer>,
    mut state: ResMut<State<GameState>>,
) {
    // Top bar
    egui::TopBottomPanel::top("cool panel").show(&egui_context.ctx_mut(), |ui| {
        if ui.button("-").clicked() {
            speed.0 = match speed.0 {
                0 => 0,
                i => i - 1,
            };

            if speed.0 == 0 && *state.current() != GameState::Paused {
                state.set(GameState::Paused).unwrap();
            }

            let adj_ms = (6 - speed.0) * TICK_LENGTH;
            timer.0.set_duration(Duration::from_millis(adj_ms));
        }
        ui.label(format!("Speed {}", speed.0));
        if ui.button("+").clicked() {
            speed.0 = match speed.0 {
                5 => 5,
                i => i + 1,
            };

            if *state.current() != GameState::Playing {
                state.set(GameState::Playing).unwrap();
            }

            let adj_ms = (6 - speed.0) * TICK_LENGTH;
            timer.0.set_duration(Duration::from_millis(adj_ms));
        }
    });
}
