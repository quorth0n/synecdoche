use std::time::Duration;

use crate::components::Province;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};

mod components;

const TICK_LENGTH: f32 = 0.25;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TickTimer(Timer::from_seconds(0.5, true)))
        .insert_resource(GameSpeed(0))
        .add_startup_system(setup_system)
        .add_state(GameState::Playing)
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(greet_people))

        .add_plugin(EguiPlugin)
        .add_system(paint_ui)
        .run();
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Paused,
    Playing 
}

struct GameSpeed(u32);

struct TickTimer(Timer);

fn greet_people(
    time: Res<Time>, mut timer: ResMut<TickTimer>, speed: Res<GameSpeed>, mut query: Query<&mut Province>) {
    let adj_delta = (time.delta().as_millis() as f64 * (speed.0 as f64 / 5.0)) as u64;
    let tick = Duration::from_millis(adj_delta);

    // tick our timer with elapsed time, adjusted for game speed
    if timer.0.tick(tick).just_finished() {
        for mut city in query.iter_mut() {
            city.pops += 100;
            println!("hello {}!", city.pops);
        }
    }
}

fn setup_system(mut commands: Commands) {
    commands.spawn().insert(Province {pops: 10000});
}

fn paint_ui(mut egui_context: ResMut<EguiContext>, mut speed: ResMut<GameSpeed>, mut commands: Commands) {
    egui::CentralPanel::default().show(&egui_context.ctx_mut(), |ui| {
        if ui.button("-").clicked() {
            speed.0 = match speed.0 {
                0 => 0,
                i => i - 1
            };
            commands.insert_resource(TickTimer(Timer::from_seconds((speed.0 as f32 * TICK_LENGTH).min(TICK_LENGTH), true)));
        }
        ui.label(format!("Speed {}", speed.0));
        if ui.button("+").clicked() {
            speed.0 = match speed.0 {
                5 => 5,
                i => i + 1
            };
            commands.insert_resource(TickTimer(Timer::from_seconds((speed.0 as f32 * TICK_LENGTH).min(TICK_LENGTH), true)));
        }
    });
}
