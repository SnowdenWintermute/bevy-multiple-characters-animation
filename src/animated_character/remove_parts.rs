use super::spawn_characters::{PlayerCharacterName, SpawnCharacterState};
use crate::asset_loader::AssetPack;
use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
    render::mesh::skinning::SkinnedMesh,
};

pub fn remove_parts(
    asset_pack: Res<AssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
    mut commands: Commands,
    scene_query: Query<(Entity, &PlayerCharacterName), With<PlayerCharacterName>>,
    children: Query<&Children>,
    parents: Query<&Parent>,
    material_handles: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    assets_gltfmesh: Res<Assets<GltfMesh>>,
    skinned_meshes: Res<Assets<SkinnedMesh>>,
    names: Query<&Name>,
    mut next_state: ResMut<NextState<SpawnCharacterState>>,
) {
    for a in skinned_meshes.iter() {
        println!("{:#?}", a)
    }

    let scene_entity_option = {
        let mut to_return = None;
        for (entity, character_name) in scene_query.iter() {
            if character_name.0 == "Casual" {
                to_return = Some(entity)
            }
        }
        to_return
    };
    if let Some(scene_entity) = scene_entity_option {
        for (i, entity) in children.iter_descendants(scene_entity).enumerate() {
            let name = match names.get(entity) {
                Ok(name) => format!("{name}"),
                Err(_) => "".to_string(),
            };

            if name == "Casual_Body" {
                commands.entity(entity).despawn_recursive();

                if let Some(gltf_loading_tracker) = &asset_pack.0.get("Adventurer") {
                    if let Some(gltf) = assets_gltf.get(&gltf_loading_tracker.gltf_handle) {
                        // for (mesh_name, mesh) in &gltf.named_meshes {
                        //     println!("mesh: {mesh_name}")
                        // }
                        // let gltf_mesh_option = assets_gltfmesh.get(&gltf.named_meshes["Cube.051"]);

                        // if let Some(gltf_mesh) = gltf_mesh_option {
                        //     let handle = commands.spawn(SkinnedMesh {
                        //         mesh: gltf_mesh.primitives[0].mesh.clone(),
                        //         material: gltf_mesh.primitives[0].material.clone().unwrap(),
                        //         transform: Transform::from_xyz(1.0, 1.0, 1.0),
                        //         global_transform: GlobalTransform::from_xyz(1.0, 1.0, 1.0),
                        //         ..Default::default()
                        //     });
                        // }
                    }
                }
            }

            // if let Ok(mesh_handle) = mesh_handles.get(entity) {
            //     for child in children.iter_descendants(entity) {}
            //     if let Ok(ref_to_parent) = parents.get(entity) {
            //         let parent = ref_to_parent.get();
            //         if let Ok(ref_to_parent_parent) = parents.get(parent) {
            //             let parent_parent = ref_to_parent_parent.get();
            //         }
            //     }
            // }
        }
    }
}
