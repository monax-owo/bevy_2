use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::player::Player;

#[derive(Default, Component, Reflect)]
pub(super) struct GroundSensor {
  pub grounded: bool,
}

pub(super) fn update_movement(
  key: Res<ButtonInput<KeyCode>>,
  time: Res<Time>,
  mut player_query: Query<(
    &mut Player,
    &Transform,
    &mut KinematicCharacterController,
    &GroundSensor,
  )>,
) {
  const GRAVITY: f32 = 9.8;
  const JUMP_GRAVITY: f32 = -64.0;

  if let Ok((mut player, player_transform, mut controller, ground_sensor)) =
    player_query.get_single_mut()
  {
    // Vec3(x,y,z) Vec2(x,z)
    let mut direction = Vec2::ZERO;

    if key.pressed(KeyCode::KeyW) {
      direction += player_transform.forward().xz();
    }

    if key.pressed(KeyCode::KeyA) {
      direction += player_transform.left().xz();
    }

    if key.pressed(KeyCode::KeyS) {
      direction += player_transform.back().xz();
    }

    if key.pressed(KeyCode::KeyD) {
      direction += player_transform.right().xz();
    }

    direction = direction.clamp_length(0.0, 1.0) * player.horizontal_speed;

    // 定数に分ける
    // 地面に付いて無いときは重力を加える
    if ground_sensor.grounded {
      player.gravity =
        (player.gravity - player.vertical_speed * 2.2 * time.delta_seconds()).clamp(9.8, 20.0);
      if key.pressed(KeyCode::Space) {
        player.gravity += JUMP_GRAVITY;
      }
    } else {
      player.gravity = (player.gravity + GRAVITY * player.vertical_speed * time.delta_seconds())
        .clamp(-500.0, 500.0);
    }

    player.direction.y -= player.gravity * 0.2;

    player.direction =
      Vec3::from((direction.x, player.direction.y, direction.y)) * time.delta_seconds();

    controller.translation = Some(player.direction);
  }
}

pub(super) fn update_grounded(
  rapier_context: Res<RapierContext>,
  mut ground_sensor_query: Query<(&mut GroundSensor, &Transform)>,
) {
  const HALF_HEIGHT: f32 = 0.2;
  const RADIUS: f32 = 0.16;

  // ray castでも良さそう？
  for (mut ground_sensor, transform) in ground_sensor_query.iter_mut() {
    ground_sensor.grounded = rapier_context
      .cast_shape(
        transform
          .translation
          .with_y(transform.translation.y - 1.4 + HALF_HEIGHT),
        Quat::default(),
        -Vec3::Y,
        &Collider::cylinder(HALF_HEIGHT, RADIUS),
        ShapeCastOptions {
          max_time_of_impact: 0.06,
          ..default()
        },
        QueryFilter::exclude_kinematic(),
      )
      .is_some();
  }
}
