use bevy::prelude::*;

mod snake;
pub use snake::snake_system;

const MAP_WIDTH: u32 = 10;
const MAP_HEIGHT: u32 = 10;

fn main() {
    App::new()
		.add_startup_system(setup_camera)
		.add_startup_system(snake_system::spawn_snake)
		.add_system(snake_system::snake_movement)
        .add_plugins(DefaultPlugins)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}