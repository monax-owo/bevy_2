use bevy::{color::palettes::css, prelude::*};

// TODO:Bundle化して公開範囲を狭める
#[derive(Component, Reflect, Debug)]
pub struct RaycastBullet {
  pub axis: Dir3,
  /// m/sec
  pub speed: f32,
  /// sec
  pub lifetime: f32,
}

// TODO:公開範囲を狭める
#[derive(Resource, Debug, Default)]
pub struct RaycastBulletAssets {
  pub bullet_mesh: Handle<Mesh>,
  pub bullet_material: Handle<StandardMaterial>,
}

pub(super) fn init_raycast_bullet(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mesh = meshes.add(Cuboid::from_length(0.2));
  let material = materials.add(Color::Srgba(css::BROWN));
  commands.insert_resource(RaycastBulletAssets {
    bullet_mesh: mesh,
    bullet_material: material,
  });
}

// TODO:ヒットスキャン
pub(super) fn update_raycast_bullet(
  mut commands: Commands,
  time: Res<Time>,
  mut bullet_query: Query<(Entity, &mut RaycastBullet, &mut Transform)>,
) {
  // TODO:↓これはプロジェクタイルのやつ
  for (entity, mut bullet, mut transform) in bullet_query.iter_mut() {}
  //   transform.translation += bullet.axis * bullet.speed * time.delta_seconds();
  //   bullet.lifetime -= time.delta_seconds();

  //   if bullet.lifetime <= 0.0 {
  //     commands.entity(entity).despawn();
  //   }
}