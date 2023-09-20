use crate::components::*;
use crate::BALLS_Z_INDEX;
use bevy::utils::Instant;
use bevy::window::{PrimaryWindow, Window};
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;
use std::ops::Mul;

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
) {
    // Very hacky solution to prevent double spawning
    static mut LAST_CLICK: Option<Instant> = None;
    unsafe {
        if let Some(last_click) = LAST_CLICK {
            if last_click.elapsed().as_secs_f32() < 0.1 {
                return;
            }
        }
        LAST_CLICK = Some(Instant::now());
    }

    let mut rng = rand::thread_rng();

    let color = Color::hsl(rng.gen_range(0.0..=360.0), 1.0, 0.5)
        .as_rgba()
        .mul(3.0);

    let size = rng.gen_range(50.0..200.0);
    let mass = size / 10.0;

    let pos = get_real_world_pos(q_windows, camera_q);
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
    mut balls_query: Query<(Entity, &Transform, With<Ball>)>,
) {
    static mut LAST_CLICK: Option<Instant> = None;
    unsafe {
        if let Some(last_click) = LAST_CLICK {
            if last_click.elapsed().as_secs_f32() < 0.1 {
                return;
            }
        }
        LAST_CLICK = Some(Instant::now());
    }
    let pos = get_real_world_pos(q_windows, camera_q);

    for (entity, transform, _) in balls_query.iter() {
        if transform.translation.truncate().distance(pos) < transform.scale.x / 2.0 {
            commands.entity(entity).despawn_recursive();
            return;
        }
    }
}
