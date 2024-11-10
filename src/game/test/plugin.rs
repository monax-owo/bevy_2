use bevy::prelude::*;

use super::{
  world::{generate_collider, init_world},
  TestTag,
};

pub struct TestPlugin;

impl Plugin for TestPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, (init_world,))
      .add_systems(Update, (generate_collider,))
      .register_type::<TestTag>();
  }
}
