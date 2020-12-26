use bevy::{
    prelude::*,
    render::pass::ClearColor,
};

/// An implementation of the classic game "Breakout"
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_startup_system(setup.system())
        .add_system(paddle_movement_system.system())
        .add_system(paddle_resize_system.system())
        .add_system(firework_shoot_system.system())
        .run();
}

struct Firework {
    alive: bool,
    velocity: Vec2,
}

impl Firework {
    fn increase_velocity(&mut self) {
        self.velocity.x += 1.0;
        self.velocity.y += 1.0;
    }
}


struct Paddle {
    speed: f32,
    size_multiplier: f32,
}

impl Paddle {
    fn bigger(&mut self) {
        self.size_multiplier += 1.0;
    }
}

fn setup(
    commands: &mut Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Add the game's entities to our world
    commands
        // cameras
        .spawn(Camera2dBundle::default())
        .spawn(CameraUiBundle::default())
        // paddle
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(0.5, 0.5, 1.0).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(Paddle {
            speed: 500.0,
            size_multiplier: 1.1,
        })
        // firework
        .spawn(SpriteBundle {
            material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -50.0, 0.0)),
            sprite: Sprite::new(Vec2::new(30.0, 30.0)),
            ..Default::default()
        })
        .with(Firework {
            alive: true,
            velocity: Vec2::new(0.5, 0.5),
        });
}

fn paddle_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
) {
    let mut direction = Vec2::new(0.0, 0.0);
    for (paddle, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Left) {
            direction[0] -= 1.0;
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction[0] += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction[1] += 1.0;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction[1] -= 1.0;
        }

        let translation = &mut transform.translation;
        // move the paddle horizontally
        translation.x += time.delta_seconds() * direction[0] * paddle.speed;
        // move the paddle vertically
        translation.y += time.delta_seconds() * direction[1] * paddle.speed;
        // bound the paddle within the walls
        translation.x = translation.x.min(380.0).max(-380.0);
        translation.y = translation.y.min(220.0).max(-220.0);
    }
}

fn paddle_resize_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Sprite)>,
) {
    for (paddle, mut sprite) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::E) {
            let owo = &mut sprite.size;
            owo.x *= paddle.size_multiplier;
            owo.y *= paddle.size_multiplier;
        }

        if keyboard_input.pressed(KeyCode::Q) {
            let owo = &mut sprite.size;
            owo.x /= paddle.size_multiplier;
            owo.y /= paddle.size_multiplier;
        }
    }
}

fn firework_shoot_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Firework, &mut Transform)>,
) {
    for (firework, mut transform) in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::Space) {
            let translation = &mut transform.translation;
            translation.x += firework.velocity.x;
            translation.y += firework.velocity.y;
        }
    }
}
