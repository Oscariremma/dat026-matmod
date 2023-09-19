use crate::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn check_for_edge_collisions(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut balls_query: Query<(&mut Transform, &mut Velocity, &Ball)>,
    time_step: Res<FixedTime>,
) {
    let window = window_query.get_single().expect("Window should exist.");

    let window_left = -window.width() / 2.0;
    let window_right = window.width() / 2.0;
    let window_top = window.height() / 2.0;
    let window_bottom = -window.height() / 2.0;

    for (transform, mut velocity, _) in &mut balls_query {
        let new_x = new_coordinate(
            transform.translation.x,
            velocity.x,
            transform.scale.x,
            &time_step,
        );

        let new_y = new_coordinate(
            transform.translation.y,
            velocity.y,
            transform.scale.y,
            &time_step,
        );

        if new_x > window_right || new_x < window_left {
            velocity.x *= -1.0;
        }

        if new_y > window_top || new_y < window_bottom {
            velocity.y *= -1.0;
        }
    }
}

fn new_coordinate(translation: f32, velocity: f32, scale: f32, time_step: &Res<FixedTime>) -> f32 {
    let x_change = translation + velocity * time_step.period.as_secs_f32();

    if x_change > 0.0 {
        x_change + scale / 2.0
    } else {
        x_change - scale / 2.0
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}
