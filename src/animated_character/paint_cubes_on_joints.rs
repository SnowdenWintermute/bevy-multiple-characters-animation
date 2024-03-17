use super::spawn_scenes::SceneName;
use bevy::prelude::*;
use bevy_mod_billboard::prelude::*;

pub fn paint_cubes_on_joints(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    scene_query: Query<Entity, With<SceneName>>,
    children: Query<&Children>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
    names: Query<&Name>,
) {
    let mut y = 0.01;
    for scene_entity in &scene_query {
        let cube_color = Color::rgb(1.0, 0.0, 0.0);
        for (_, entity) in children.iter_descendants(scene_entity).enumerate() {
            let name = match names.get(entity) {
                Ok(name) => format!("{name}"),
                Err(_) => "unnamed".to_string(),
            };

            // NON_MESH ENTITY;
            if let Err(_) = mesh_handles.get(entity) {
                if let Ok(_) = global_transforms.get(entity) {
                    // println!("NAMED NON MESH NODE: {:#?}", name);
                    let cube_handle = meshes.add(Cuboid::new(0.01, 0.01, 0.01));
                    let cube_material_handle = materials.add(StandardMaterial {
                        base_color: cube_color.clone(),
                        ..default()
                    });

                    // LABEL IT
                    let fira_sans_regular_handle = asset_server.load("FiraSans-Regular.ttf");
                    let mut billboard_entity_commands = commands.spawn(BillboardTextBundle {
                        transform: Transform::from_xyz(0.02, y, 0.0)
                            .with_scale(Vec3::splat(0.0005)),
                        text: Text::from_sections([TextSection {
                            value: format!("{}", name),
                            style: TextStyle {
                                font_size: 60.0,
                                font: fira_sans_regular_handle.clone(),
                                color: Color::WHITE,
                            },
                        }]),
                        ..default()
                    });
                    billboard_entity_commands.set_parent(entity);

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

        // y += 0.02;
    }
}
