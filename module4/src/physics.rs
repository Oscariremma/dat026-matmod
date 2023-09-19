use crate::components::*;
use bevy::prelude::*;
use bevy_prototype_debug_lines::*;
use std::ops::{Add, Mul};

use bevy::window::PrimaryWindow;

pub fn handle_for_edge_collisions(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut balls_query: Query<(&Transform, &mut Velocity, &Ball)>,
    time_step: Res<FixedTime>,
) {
    let window = window_query.get_single().expect("Window should exist.");

    let window_left = -window.width() / 2.0;
    let window_right = window.width() / 2.0;
    let window_top = window.height() / 2.0;
    let window_bottom = -window.height() / 2.0;

    for (transform, mut velocity, _) in &mut balls_query {
        let new_x = next_frame_edge(
            transform.translation.x,
            velocity.x,
            transform.scale.x,
            &time_step,
        );

        let new_y = next_frame_edge(
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

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

pub fn handle_inter_ball_collision(
    mut balls_query: Query<(&Transform, &mut Velocity, &Ball)>,
    time_step: Res<FixedTime>,
    mut lines: ResMut<DebugLines>,
) {
    let mut ball_combinations = balls_query.iter_combinations_mut();

    while let Some([mut ball_a, mut ball_b]) = ball_combinations.fetch_next() {
        if is_colliding(&time_step, &ball_a, &ball_b) {
            println!("Collision!");
            bounce(&mut ball_a, &mut ball_b, &mut lines);
        }
    }
}

fn is_colliding(
    time_step: &Res<FixedTime>,
    ball_a: &(&Transform, Mut<Velocity>, &Ball),
    ball_b: &(&Transform, Mut<Velocity>, &Ball),
) -> bool {
    let (transform_a, velocity_a, _) = ball_a;
    let (transform_b, velocity_b, _) = ball_b;

    let next_position_a = next_frame_coords(
        &transform_a.translation.truncate(),
        &velocity_a.0,
        time_step,
    );

    let next_position_b = next_frame_coords(
        &transform_b.translation.truncate(),
        &velocity_b.0,
        time_step,
    );

    let next_distance = next_position_a.distance(next_position_b);

    let combined_size = (transform_a.scale.x + transform_b.scale.x) / 2.0;
    let next_edge_distance = next_distance - combined_size;

    next_edge_distance <= 0.0
}

fn bounce(
    ball_a: &mut (&Transform, Mut<Velocity>, &Ball),
    ball_b: &mut (&Transform, Mut<Velocity>, &Ball),
    lines: &mut ResMut<DebugLines>,
) {
    let (transform_a, velocity_a, _) = ball_a;
    let (transform_b, velocity_b, _) = ball_b;

    let norm_vector =
        (transform_b.translation.truncate() - transform_a.translation.truncate()).normalize();

    velocity_a.0 = velocity_a.0.length() * -norm_vector;
    velocity_b.0 = velocity_b.0.length() * norm_vector;

    lines.line_colored(
        transform_a.translation.truncate().extend(0.0),
        transform_a
            .translation
            .truncate()
            .add(norm_vector.mul(200.0))
            .extend(0.0),
        10.0,
        Color::RED,
    );
}

fn next_frame_edge(translation: f32, velocity: f32, scale: f32, time_step: &Res<FixedTime>) -> f32 {
    let change = translation + velocity * time_step.period.as_secs_f32();

    if change > 0.0 {
        change + scale / 2.0
    } else {
        change - scale / 2.0
    }
}

fn next_frame_coords(cords: &Vec2, velocity: &Vec2, time_step: &Res<FixedTime>) -> Vec2 {
    (*cords) + ((*velocity) * time_step.period.as_secs_f32())
}
