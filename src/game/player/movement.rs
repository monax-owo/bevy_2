use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::{input::PlayerInput, Player, PLAYER_HEIGHT, PLAYER_OFFSET};

pub const GRAVITY: f32 = 9.8;

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

// ユーザーからの入力を反映する
pub(super) fn update_movement_input(
  keyboard_input: Res<ButtonInput<KeyCode>>,
  key: Res<PlayerInput>,
  mut player_query: Query<(&mut Player, &GroundSensor)>,
) {
  const JUMP_HEIGHT: f32 = -80.0;

  if let Ok((mut player, ground_sensor)) = player_query.get_single_mut() {
    if keyboard_input.pressed(key.forward) {
      player.direction.x += 1.0;
    }

    if keyboard_input.pressed(key.left) {
      player.direction.z += -1.0;
    }

    if keyboard_input.pressed(key.back) {
      player.direction.x += -1.0;
    }

    if keyboard_input.pressed(key.right) {
      player.direction.z += 1.0;
    }

    // TODO:プレイヤーが止まったら歩きの速度にする
    if keyboard_input.pressed(key.dash) {
      player.horizontal_speed = 20.0;
    }

    if ground_sensor.grounded && player.vertical_accel > 0.0 && keyboard_input.pressed(key.jump) {
      // 重力とJUMP_HEIGHTで打ち消されないようにする
      if player.vertical_accel > GRAVITY {
        player.vertical_accel = GRAVITY;
      }

      player.vertical_accel += JUMP_HEIGHT;
    }
  }
}

// Playerのプロパティを使用してエンティティを移動させる
pub(super) fn update_movement(
  time: Res<Time>,
  mut player_query: Query<(
    &mut Player,
    &Transform,
    &mut KinematicCharacterController,
    &GroundSensor,
  )>,
) {
  if let Ok((mut player, player_transform, mut controller, ground_sensor)) =
    player_query.get_single_mut()
  {
    player.direction = player.direction.x * player_transform.forward()
      + player.direction.z * player_transform.right();

    if ground_sensor.grounded && player.vertical_accel >= 0.0 {
      // 弱い重力を加える
      player.vertical_accel = (player.vertical_accel
        - player.vertical_speed * 6.0 * time.delta_seconds())
      .clamp(GRAVITY, 500.0);
    } else {
      // 重力を加える
      player.vertical_accel = (player.vertical_accel
        + GRAVITY * player.vertical_speed * time.delta_seconds())
      .clamp(-500.0, 500.0);
    }

    player.direction.y -= player.vertical_accel * 0.2;

    let translation = (player.direction * player.horizontal_speed).with_y(player.direction.y)
      * time.delta_seconds();

    controller.translation = Some(translation);
    player.direction = Vec3::ZERO;
  }
}

pub(super) fn update_grounded(
  rapier_context: Res<RapierContext>,
  mut ground_sensor_query: Query<(&mut GroundSensor, &Transform)>,
) {
  // ray castでも良さそう？->ray castにした
  // todo:おかしかったらshape castに戻す
  for (mut ground_sensor, transform) in ground_sensor_query.iter_mut() {
    ground_sensor.grounded = rapier_context
      .cast_ray(
        transform
          .translation
          .with_y(transform.translation.y - PLAYER_HEIGHT + PLAYER_OFFSET),
        -Vec3::Y,
        ground_sensor.toi,
        true,
        QueryFilter::exclude_kinematic(),
      )
      .is_some();
  }
}
