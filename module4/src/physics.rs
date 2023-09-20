use crate::components::*;
use bevy::prelude::*;

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
        let new_edge_x = next_frame_edge(
            transform.translation.x,
            velocity.x,
            transform.scale.x,
            &time_step,
        );

        let new_edge_y = next_frame_edge(
            transform.translation.y,
            velocity.y,
            transform.scale.y,
            &time_step,
        );

        if new_edge_x > window_right || new_edge_x < window_left {
            velocity.x *= -1.0;
        }

        if new_edge_y > window_top || new_edge_y < window_bottom {
            velocity.y *= -1.0;
        }
    }
}

pub fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity, &Ball)>,
    time_step: Res<FixedTime>,
) {
    for (mut transform, velocity, ball) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

pub fn apply_gravity(
    mut query: Query<&mut Velocity>,
    gravity: Res<Gravity>,
    time_step: Res<FixedTime>,
) {
    for mut velocity in &mut query {
        velocity.y -= gravity.0 * time_step.period.as_secs_f32();
    }
}

pub fn handle_inter_ball_collision(
    mut balls_query: Query<(&Transform, &mut Velocity, &Ball)>,
    time_step: Res<FixedTime>,
) {
    let mut ball_combinations = balls_query.iter_combinations_mut();

    while let Some([mut ball_a, mut ball_b]) = ball_combinations.fetch_next() {
        if is_colliding(&time_step, &ball_a, &ball_b) {
            bounce(&mut ball_a, &mut ball_b);
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
) {
    let (transform_a, velocity_a, ball_a) = ball_a;
    let (transform_b, velocity_b, ball_b) = ball_b;

    let vec_between =
        (transform_b.translation.truncate() - transform_a.translation.truncate()).normalize();

    let vel_a_project = velocity_a.0.project_onto(vec_between);
    let vel_b_project = velocity_b.0.project_onto(vec_between);

    velocity_a.0 -= vel_a_project;
    velocity_b.0 -= vel_b_project;

    let new_speed_a_project =
        calculate_speed_after_collision(ball_a.mass, ball_b.mass, vel_a_project, vel_b_project);
    let new_speed_b_project =
        calculate_speed_after_collision(ball_b.mass, ball_a.mass, vel_b_project, vel_a_project);

    velocity_a.0 += new_speed_a_project;
    velocity_b.0 += new_speed_b_project;
}

fn next_frame_edge(center: f32, velocity: f32, diameter: f32, time_step: &Res<FixedTime>) -> f32 {
    let change = center + velocity * time_step.period.as_secs_f32();

    calculate_edge(change, diameter)
}

fn calculate_edge(center: f32, diameter: f32) -> f32 {
    if center > 0.0 {
        center + diameter / 2.0
    } else {
        center - diameter / 2.0
    }
}

fn next_frame_coords(cords: &Vec2, velocity: &Vec2, time_step: &Res<FixedTime>) -> Vec2 {
    (*cords) + ((*velocity) * time_step.period.as_secs_f32())
}

fn calculate_speed_after_collision(m_a: f32, m_b: f32, u_a: Vec2, u_b: Vec2) -> Vec2 {
    (m_a - m_b) / (m_a + m_b) * u_a + (2.0 * m_b) / (m_a + m_b) * u_b
}
