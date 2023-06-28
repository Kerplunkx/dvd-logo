use bevy::{math::vec3, prelude::*, window::PrimaryWindow};
use rand::Rng;

const LOGO_WIDTH: f32 = 1024.0;
const LOGO_HEIGHT: f32 = 589.0;
const LOGO_SPEED: f32 = 200.0;
const SCALE: f32 = 0.15;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_logo)
        .add_system(logo_movement)
        .add_system(update_logo_direction)
        .run()
}

#[derive(Component)]
struct Logo {
    direction: Vec3,
}

fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

fn spawn_logo(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("images/dvd.png"),
            transform: Transform {
                translation: vec3(window.width() / 2.0, window.height() / 2.0, 0.0),
                rotation: Quat::from_xyzw(0.0, 0.0, 0.0, 0.0),
                scale: Vec3::splat(SCALE),
            },
            ..default()
        },
        Logo {
            direction: vec3(1.0, 1.0, 0.0),
        },
    ));
}

fn logo_movement(time: Res<Time>, mut logo_query: Query<(&mut Transform, &Logo)>) {
    for (mut transform, logo) in logo_query.iter_mut() {
        let direction = Vec3::new(logo.direction.x, logo.direction.y, 0.0);
        transform.translation += direction * LOGO_SPEED * time.delta_seconds();
    }
}

fn update_logo_direction(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut logo_query: Query<(&Transform, &mut Logo, &mut Sprite)>,
) {
    let window = window_query.get_single().unwrap();
    let x_min = (LOGO_WIDTH * SCALE) / 2.0;
    let x_max = window.width() - (LOGO_WIDTH * SCALE) / 2.0;
    let y_min = (LOGO_HEIGHT * SCALE) / 2.0;
    let y_max = window.height() - (LOGO_HEIGHT * SCALE) / 2.0;

    let mut is_bouncing = false;

    for (transform, mut logo, mut sprite) in logo_query.iter_mut() {
        let translation = transform.translation;
        if translation.x > x_max || translation.x < x_min {
            logo.direction.x *= -1.0;
            is_bouncing = true;
        }
        if translation.y > y_max || translation.y < y_min {
            logo.direction.y *= -1.0;
            is_bouncing = true;
        }

        if is_bouncing {
            sprite.color = Color::rgb(
                rand::thread_rng().gen_range(0.0..1.0),
                rand::thread_rng().gen_range(0.0..1.0),
                rand::thread_rng().gen_range(0.0..1.0),
            );
        }
    }
}
