use crate::components::*;
use crate::physics::is_approx_colliding;
use crate::BALLS_Z_INDEX;
use bevy::utils::Instant;
use bevy::window::{PrimaryWindow, Window};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;
use std::f32::consts::PI;
use std::ops::Mul;

macro_rules! debounce_return {
    () => {
        static mut LAST_CLICK: Option<Instant> = None;
        unsafe {
            if let Some(last_click) = LAST_CLICK {
                if last_click.elapsed().as_secs_f32() < 0.1 {
                    return;
                }
            }
            LAST_CLICK = Some(Instant::now());
        }
    };
}

pub fn get_real_world_pos(
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) -> Vec2 {
    let (camera, camera_transform) = camera_q.single();

    if let Some(world_position) = q_windows
        .single()
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        return world_position;
    }

    return Vec2::new(0.0, 0.0);
}

pub fn handle_left_click(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    balls_query: Query<(&Transform, With<Ball>)>,
) {
    debounce_return!();

    let pos = get_real_world_pos(q_windows, camera_q);

    let mut rng = rand::thread_rng();
    let size = rng.gen_range(50.0..200.0);

    for (transform, _) in balls_query.iter() {
        if is_approx_colliding(
            transform.translation.truncate(),
            transform.scale.x,
            pos,
            size,
        ) {
            return;
        }
    }

    let color = Color::hsl(rng.gen_range(0.0..=360.0), 1.0, 0.5)
        .as_rgba()
        .mul(3.0);

    let mass = PI * (size / 2.0).powi(2);

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(color)),
            transform: Transform::from_translation(pos.extend(BALLS_Z_INDEX))
                .with_scale(Vec3::splat(size)),
            ..default()
        },
        Ball { mass },
        Velocity(Vec2::new(0.0, 0.0)),
    ));
}

pub fn handle_right_click(
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    balls_query: Query<(Entity, &Transform, With<Ball>)>,
) {
    debounce_return!();
    let pos = get_real_world_pos(q_windows, camera_q);

    for (entity, transform, _) in balls_query.iter() {
        if transform.translation.truncate().distance(pos) < transform.scale.x / 2.0 {
            commands.entity(entity).despawn_recursive();
            return;
        }
    }
}
