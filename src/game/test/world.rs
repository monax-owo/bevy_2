use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use blenvy::{BluePrintBundle, BlueprintInfo};
use serde::Deserialize;
use serde_json::Value;

pub(super) fn init_world(mut commands: Commands) {
  commands.spawn((
    BluePrintBundle {
      blueprint: BlueprintInfo::from_path("blueprints/ground.glb"),
      ..default()
    },
    Name::new("Ground"),
  ));
}

#[derive(Deserialize, Debug)]
pub struct SerDeGenerateCollider {
  collider: Option<Value>,
}

// TODO:Colliderを自動生成
pub(super) fn generate_collider(
  mut commands: Commands,
  meshes: Res<Assets<Mesh>>,
  query: Query<(Entity, &GltfExtras, &Children), Added<GltfExtras>>,
  generate_collider_query: Query<(&Name, &Handle<Mesh>), Without<Collider>>,
) {
  for (entity, gltf_extras, children) in query.iter() {
    let de = serde_json::from_str::<SerDeGenerateCollider>(&gltf_extras.value)
      .expect("failure parse GltfExtras");

    println!("{} {}", entity.index(), de.collider.is_some());

    // let option = ComputedColliderShape::ConvexDecomposition(VHACDParameters {
    //   concavity: 0.01,
    //   resolution: 64,
    //   ..default()
    // });

    let option = ComputedColliderShape::TriMesh;

    for &child in children.iter() {
      if let Ok((name, mesh)) = generate_collider_query.get(child) {
        println!("{} {:?}", name.as_str(), mesh);

        commands
          .entity(child)
          .insert(Collider::from_bevy_mesh(meshes.get(mesh).unwrap(), &option).unwrap());
      }
    }
  }
}
