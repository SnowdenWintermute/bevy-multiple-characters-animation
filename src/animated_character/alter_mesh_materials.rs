use super::spawn_scenes::SceneName;
use bevy::prelude::*;

pub fn alter_mesh_materials(
    scene_query: Query<Entity, With<SceneName>>,
    children: Query<&Children>,
    material_handles: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_handles: Query<&Handle<Mesh>>,
    meshes: ResMut<Assets<Mesh>>,
    names: Query<&Name>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    // DRAW AABB
    config_store.config_mut::<AabbGizmoConfigGroup>().1.draw_all = true;
    // ALTER TRANSPARENCY
    for scene_entity in &scene_query {
        for (_, entity) in children.iter_descendants(scene_entity).enumerate() {
            let name = match names.get(entity) {
                Ok(name) => format!("{name}"),
                Err(_) => "".to_string(),
            };

            if let Ok(mesh_handle) = mesh_handles.get(entity) {
                if let Some(_) = meshes.get(mesh_handle) {
                    // println!("NAMED MESH NODE: {:#?}", name);
                    // println!("mesh: {:#?}", mesh);
                    if let Ok(material_handle) = material_handles.get(entity) {
                        if let Some(material) = materials.get_mut(material_handle) {
                            material.alpha_mode = AlphaMode::Blend;
                            material.base_color.set_a(0.55);
                        }
                    }
                }
            }
        }
    }
}
