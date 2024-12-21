// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use bevy::prelude::*;

use avian3d::{
    prelude::PhysicsDebugPlugin, 
    PhysicsPlugins
};
mod shared;
mod camera;
mod test;
mod fractal;
mod valley;
mod player;
mod ui;
mod animator;
mod map;
mod target_select;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Loading,
    Game,
    Map
}

#[derive(Component)]
pub struct NotReady;


fn main() {
    App::new()
    .insert_resource(ClearColor(Color::BLACK))
    .add_plugins((
        DefaultPlugins,
        camera::CameraPlugin,
        valley::ValleyPlugin,
        player::PlayerPlugin,
        fractal::FractalPlugin,
        ui::UIPlugin,
        animator::AnimatorPlugin,
        map::MapPlugin,
        target_select::TargetSelectPlugin
    ))
    .init_state::<GameState>()
    .add_systems(Update, check_ready.run_if(in_state(GameState::Loading)))
    .add_plugins((
        PhysicsPlugins::default(),
        // PhysicsDebugPlugin::default(),
    ))
    
    .run();

}    

// ----

fn check_ready(
    not_ready_q: Query<&NotReady>,
    mut next: ResMut<NextState<GameState>>     
) {
    if not_ready_q.is_empty() {
        println!("GAME!");
        next.set(GameState::Game);
    }
}

