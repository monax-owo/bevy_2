use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{input::PlayerInput, Player};

#[derive(Component, Reflect)]
pub(super) struct GroundSensor {
  /// 接地しているか
  pub grounded: bool,
  /// time-of-impact
  pub toi: f32,
}

impl Default for GroundSensor {
  fn default() -> Self {
    Self {
      grounded: Default::default(),
      // TODO:調整する
      toi: 0.16,
    }
  }
}

pub(super) fn update_movement(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  key: Res<PlayerInput>,
  time: Res<Time>,
  mut player_query: Query<(
    &mut Player,
    &Transform,
    &mut KinematicCharacterController,
    &GroundSensor,
  )>,
) {
  const JUMP_HEIGHT: f32 = -80.0;

  if let Ok((mut player, player_transform, mut controller, ground_sensor)) =
    player_query.get_single_mut()
  {
    let mut direction = Vec3::ZERO;

    if keyboard_input.pressed(key.forward) {
      direction.x += 1.0;
    }

    if keyboard_input.pressed(key.left) {
      direction.z += -1.0;
    }

    if keyboard_input.pressed(key.back) {
      direction.x += -1.0;
    }

    if keyboard_input.pressed(key.right) {
      direction.z += 1.0;
    }

    // TODO:プレイヤーが止まったら歩きの速度にする
    if keyboard_input.pressed(key.dash) {
      player.horizontal_speed = 20.0;
    }

    direction = direction.x * player_transform.forward() + direction.z * player_transform.right();

    // jump
    if ground_sensor.grounded && keyboard_input.pressed(key.jump) {
      player.vertical_accel += JUMP_HEIGHT;
    }

    player.direction =
      (direction * player.horizontal_speed).with_y(player.direction.y) * time.delta_seconds();

    controller.translation = Some(player.direction);
  }
}

pub(super) fn update_gravity(
  time: Res<Time>,
  mut player_query: Query<(
    &mut Player,
    &Transform,
    &mut KinematicCharacterController,
    &GroundSensor,
  )>,
) {
  const GRAVITY: f32 = 9.8;

  if let Ok((mut player, player_transform, mut controller, ground_sensor)) =
    player_query.get_single_mut()
  {
    // 地面に付いて無いときは重力を加える
    if ground_sensor.grounded {
      player.vertical_accel = (player.vertical_accel
        - player.vertical_speed * 2.2 * time.delta_seconds())
      .clamp(9.8, 20.0);
    } else {
      player.vertical_accel = (player.vertical_accel
        + GRAVITY * player.vertical_speed * time.delta_seconds())
      .clamp(-500.0, 500.0);
    }

    player.direction.y -= player.vertical_accel * 0.2 * time.delta_seconds();

    controller.translation = Some(player.direction);
  }
}

pub(super) fn update_grounded(
  rapier_context: Res<RapierContext>,
  mut ground_sensor_query: Query<(&mut GroundSensor, &Transform)>,
) {
  // const HALF_HEIGHT: f32 = 0.2;
  // const RADIUS: f32 = 0.16;

  // ray castでも良さそう？->ray castにした
  // todo:おかしかったらshape castに戻す
  for (mut ground_sensor, transform) in ground_sensor_query.iter_mut() {
    ground_sensor.grounded = rapier_context
      .cast_ray(
        transform.translation.with_y(transform.translation.y - 1.4),
        -Vec3::Y,
        ground_sensor.toi,
        true,
        QueryFilter::exclude_kinematic(),
      )
      .is_some();
  }
}
