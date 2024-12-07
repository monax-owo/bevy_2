use bevy::prelude::*;
use bevy_rapier3d::{
  plugin::{NoUserData, RapierPhysicsPlugin},
  render::RapierDebugRenderPlugin,
};
use blenvy::BlenvyPlugin;

use super::{
  inventory::plugin::InventoryPlugin, player::PlayerPlugin, shooting::ShootingPlugin,
  state::StatePlugin, test::TestPlugin, ui::UiPlugin, world::WorldPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
  fn build(&self, app: &mut bevy::prelude::App) {
    let app = app
      .add_plugins((
        // RapierPhysicsPlugin::<NoUserData>::default(),
        BlenvyPlugin::default(),
        // inventory crateのPlugin
        inventory::InventoryPlugin,
        InventoryPlugin,
        PlayerPlugin,
        ShootingPlugin,
        StatePlugin,
        UiPlugin,
        WorldPlugin,
      ))
      .add_systems(Startup, setup_blenvy);

    if cfg!(debug_assertions) {
      app.add_plugins((TestPlugin, RapierDebugRenderPlugin::default()));
    }
  }
}

// blenvy
fn setup_blenvy(mut commands: Commands) {
  commands.spawn(blenvy::BluePrintBundle {
    blueprint: blenvy::BlueprintInfo::from_path("levels/World.glb"),
    ..default()
  });
}
