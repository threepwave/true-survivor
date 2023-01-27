
pub mod snake_system {

	use bevy::prelude::*;

	#[derive(Component)]
	pub struct SnakeHead;

	const SNAKE_HEAD_COLOR: Color = Color::rgb(0.7, 0.7, 0.7);

	// Creates a snake which can move around the map
	pub fn spawn_snake(mut commands: Commands) {
		commands
		.spawn(SpriteBundle {
			sprite: Sprite {
				color: SNAKE_HEAD_COLOR,
				..default()
			},
			transform: Transform {
				scale: Vec3::new(10.0, 10.0, 10.0),
				..default()
			},
			..default()
		})
		.insert(SnakeHead);
	}

	// Moves the snaek around the map
	pub fn snake_movement(keyboard_input: Res<Input<KeyCode>>,
		mut head_positions: Query<&mut Transform, With<SnakeHead>>) {
		for mut transform in head_positions.iter_mut() {
			if keyboard_input.pressed(KeyCode::Left) {
				transform.translation.x -= 2.0;
			}
			
			if keyboard_input.pressed(KeyCode::Right) {
				transform.translation.x += 2.0;
			}
			
			if keyboard_input.pressed(KeyCode::Up) {
				transform.translation.y += 2.0;
			}
			
			if keyboard_input.pressed(KeyCode::Down) {
				transform.translation.y -= 2.0;
			}
		}
	}
}