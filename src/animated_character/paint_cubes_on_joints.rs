use super::spawn_characters::PlayerCharacterName;
use crate::asset_loader::AssetPack;
use bevy::{gltf::Gltf, prelude::*, render::mesh::shape::Cube};

pub fn paint_cubes_on_joints(
    mut commands: Commands,
    scene_query: Query<Entity, With<PlayerCharacterName>>,
    children: Query<&Children>,
    material_handles: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
    names: Query<&Name>,
) {
    for scene_entity in &scene_query {
        // let mut cube_color = Color::rgb(0.0, 0.0, 0.0);
        let cube_color = Color::rgb(1.0, 0.0, 0.0);
        for (_, entity) in children.iter_descendants(scene_entity).enumerate() {
            let name = match names.get(entity) {
                Ok(name) => format!("{name}"),
                Err(_) => "".to_string(),
            };

            // NON_MESH ENTITY;
            if let Err(_) = mesh_handles.get(entity) {
                if let Ok(_) = global_transforms.get(entity) {
                    // println!("NAMED NON MESH NODE: {:#?}", name);
                    let cube_handle = meshes.add(Cube::new(0.01).into());
                    // cube_color.set_r(cube_color.r() + 0.1);
                    let cube_material_handle = materials.add(StandardMaterial {
                        base_color: cube_color.clone(),
                        ..default()
                    });

                    if let Ok(_) = transforms.get_mut(scene_entity) {
                        let mut entity_commands = commands.spawn(PbrBundle {
                            mesh: cube_handle.clone(),
                            material: cube_material_handle.clone(),
                            transform: Transform::from_xyz(0.0, 0.0, 0.0),
                            ..Default::default()
                        });
                        entity_commands.set_parent(entity);
                    }
                } else {
                    println!("NON_MESH ENTITY WITHOUT TRANSFORM??")
                }
            }
        }
    }
}
