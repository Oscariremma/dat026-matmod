use crate::components::*;
use bevy::prelude::*;

use bevy::window::PrimaryWindow;

pub fn handle_for_edge_collisions(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut balls_query: Query<(&mut Transform, &mut Velocity, &Ball)>,
    time_step: Res<FixedTime>,
) {
    let window = window_query.get_single().expect("Window should exist.");

    let window_left = -window.width() / 2.0;
    let window_right = window.width() / 2.0;
    let window_top = window.height() / 2.0;
    let window_bottom = -window.height() / 2.0;

    for (mut transform, mut velocity, _) in &mut balls_query {
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

        if new_edge_x > window_right && velocity.x > 0.0
            || new_edge_x < window_left && velocity.x < 0.0
        {
            velocity.x *= -1.0;
        }

        if new_edge_y > window_top && velocity.y > 0.0
            || new_edge_y < window_bottom && velocity.y < 0.0
        {
            velocity.y *= -1.0;
        }

        let current_edge_x = calculate_edge(transform.translation.x, transform.scale.x);
        let current_edge_y = calculate_edge(transform.translation.y, transform.scale.y);

        if current_edge_x > window_right || current_edge_x < window_left {
            transform.translation.x = window_right.copysign(transform.translation.x)
                + (transform.scale.x / 2.0).copysign(-transform.translation.x);
        }

        if current_edge_y > window_top || current_edge_y < window_bottom {
            transform.translation.y = window_top.copysign(transform.translation.y)
                + (transform.scale.y / 2.0).copysign(-transform.translation.y);
        }
    }
}

pub fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
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
    mut balls_query: Query<(&mut Transform, &mut Velocity, &Ball)>,
    time_step: Res<FixedTime>,
) {
    let mut ball_combinations = balls_query.iter_combinations_mut();

    while let Some([mut ball_a, mut ball_b]) = ball_combinations.fetch_next() {
        let (ref mut transform_a, velocity_a, _) = ball_a;
        let (ref mut transform_b, velocity_b, _) = ball_b;

        if is_colliding(
            transform_a.translation.truncate(),
            transform_a.scale.x,
            transform_b.translation.truncate(),
            transform_b.scale.x,
        ) {
            move_apart(&mut ball_a.0, &mut ball_b.0);
            continue;
        }

        let mut ball_a = (transform_a.as_ref(), velocity_a, ball_a.2);
        let mut ball_b = (transform_b.as_ref(), velocity_b, ball_b.2);

        if will_collide(&time_step, &mut ball_a, &mut ball_b) {
            bounce(&mut ball_a, &mut ball_b);
        }
    }
}

pub fn is_approx_colliding(
    center_pos_a: Vec2,
    width_a: f32,
    center_pos_b: Vec2,
    width_b: f32,
) -> bool {
    let width_a = width_a / 2.0;
    let width_b = width_b / 2.0;

    let a_min = center_pos_a - width_a;
    let a_max = center_pos_a + width_a;

    let b_min = center_pos_b - width_b;
    let b_max = center_pos_b + width_b;

    a_min.x < b_max.x && a_max.x > b_min.x && a_min.y < b_max.y && a_max.y > b_min.y
}

fn will_collide(
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

    is_colliding(
        next_position_a,
        transform_a.scale.x,
        next_position_b,
        transform_b.scale.x,
    )
}

fn is_colliding(pos_a: Vec2, width_a: f32, pos_b: Vec2, width_b: f32) -> bool {
    if !is_approx_colliding(pos_a, width_a, pos_b, width_b) {
        return false;
    }

    let distance = pos_a.distance(pos_b);

    let combined_size = (width_a + width_b) / 2.0;
    let next_edge_distance = distance - combined_size;

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

fn move_apart(transform_a: &mut Transform, transform_b: &mut Transform) {
    let vec_between = (transform_b.translation.truncate() - transform_a.translation.truncate())
        .normalize()
        .extend(0.0);

    let distance = transform_a
        .translation
        .truncate()
        .distance(transform_b.translation.truncate());

    let combined_size = (transform_a.scale.x + transform_b.scale.x) / 2.0;
    let next_edge_distance = distance - combined_size;

    let move_amount = next_edge_distance / 2.0;

    transform_a.translation += vec_between * move_amount;
    transform_b.translation -= vec_between * move_amount;
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
