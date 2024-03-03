mod link_animations;
mod run_animations;
use crate::asset_loader::{AssetLoaderState, AssetPack};
use bevy::{gltf::Gltf, pbr::Mesh3d, prelude::*, render::mesh::shape::Cube, utils::HashMap};

pub struct AnimatedCharacterPlugin;
impl Plugin for AnimatedCharacterPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SpawnCharacterState>()
            .add_systems(OnEnter(AssetLoaderState::Done), spawn_characters)
            .add_systems(
                Update,
                (
                    run_animations::run_animations.run_if(in_state(AssetLoaderState::Done)),
                    link_animations::link_animations.run_if(in_state(AssetLoaderState::Done)),
                ),
            )
            // .add_systems(OnEnter(SpawnCharacterState::Editing), remove_parts)
            .add_systems(OnEnter(SpawnCharacterState::Done), list_scene_tree);
    }
}

#[derive(Resource, Debug)]
struct Animations(HashMap<String, Handle<AnimationClip>>);

#[derive(Component, Debug)]
pub struct PlayerCharacterName(String);

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone, States)]
pub enum SpawnCharacterState {
    #[default]
    Loading,
    Editing,
    Done,
}

fn spawn_characters(
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<SpawnCharacterState>>,
) {
    let mut x_pos = -0.5;
    for (name, gltf_handle_loading_tracker) in &asset_pack.0 {
        if let Some(gltf) = assets_gltf.get(&gltf_handle_loading_tracker.gltf_handle) {
            commands.spawn((
                SceneBundle {
                    scene: gltf.named_scenes["Scene"].clone(),
                    transform: Transform::from_xyz(x_pos, 0.0, 0.0),
                    ..Default::default()
                },
                PlayerCharacterName(name.clone()),
            ));

            x_pos += 1.0;

            let mut animations = HashMap::new();
            for named_animation in gltf.named_animations.iter() {
                animations.insert(
                    named_animation.0.clone(),
                    gltf.named_animations[named_animation.0].clone(),
                );
            }

            commands.insert_resource(Animations(animations));
        }
    }
    next_state.set(SpawnCharacterState::Editing);
    // if let Some(gltf) = assets_gltf.get(&asset_pack.0) {
    //     // println!("gltf: {:#?}", gltf);
    //     for node in gltf.named_nodes.iter() {
    //         // println!("node: {:#?}", node.0);
    //     }

    //     commands.spawn((
    //         SceneBundle {
    //             scene: gltf.named_scenes["Scene"].clone(),
    //             transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //             ..Default::default()
    //         },
    //         PlayerCharacterName("Player 1".to_string()),
    //     ));
    //     next_state.set(SpawnCharacterState::Editing);

    //     // commands.spawn((
    //     //     SceneBundle {
    //     //         scene: gltf.named_scenes["Scene"].clone(),
    //     //         transform: Transform::from_xyz(2.0, 0.0, 0.0),
    //     //         ..Default::default()
    //     //     },
    //     //     PlayerCharacterName("Player 2".to_string()),
    //     // ));

    //     let mut animations = HashMap::new();
    //     for named_animation in gltf.named_animations.iter() {
    //         animations.insert(
    //             named_animation.0.clone(),
    //             gltf.named_animations[named_animation.0].clone(),
    //         );
    //     }

    //     commands.insert_resource(Animations(animations));
    // }
}

pub fn remove_parts(
    asset_pack: Res<AssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
    mut commands: Commands,
    scene_query: Query<Entity, With<PlayerCharacterName>>,
    children: Query<&Children>,
    parents: Query<&Parent>,
    material_handles: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    names: Query<&Name>,
    mut next_state: ResMut<NextState<SpawnCharacterState>>,
) {
    let scene_entity = &scene_query.get_single().expect("to have spawned a scene");
    for (i, entity) in children.iter_descendants(*scene_entity).enumerate() {
        let name = match names.get(entity) {
            Ok(name) => format!("{name}"),
            Err(_) => "".to_string(),
        };

        if name == "Casual_Body" {
            commands.entity(entity).despawn_recursive();

            // if let Some(gltf) = assets_gltf.get(&asset_pack.0) {
            //     let handle = commands.spawn((PbrBundle {
            //         mesh: gltf.named_meshes["Casual_Body"].clone(),
            //         ..Default::default()
            //     },));
            // }
        }

        if let Ok(mesh_handle) = mesh_handles.get(entity) {
            for child in children.iter_descendants(entity) {
                // println!("mesh child: {}", names.get(child).expect(""));
                //
            }
            if let Ok(ref_to_parent) = parents.get(entity) {
                let parent = ref_to_parent.get();
                // println!("mesh parent: {}", names.get(parent).expect(""));
                if let Ok(ref_to_parent_parent) = parents.get(parent) {
                    let parent_parent = ref_to_parent_parent.get();
                    // println!(
                    //     "mesh parent parent: {}",
                    //     names.get(parent_parent).expect("")
                    // );
                }
                // for child in children.iter_descendants(parent) {
                //     println!("parent child: {}", names.get(child).expect(""))
                // }
            }
        }
    }
    next_state.set(SpawnCharacterState::Done);
}

pub fn list_scene_tree(
    mut commands: Commands,
    scene_query: Query<Entity, With<PlayerCharacterName>>,
    children: Query<&Children>,
    time: Res<Time>,
    material_handles: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
    names: Query<&Name>,
    asset_pack: Res<AssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    for scene_entity in &scene_query {
        let mut num_non_meshes_seen = 0;
        let mut num_meshes_seen = 0;
        // let mut cube_color = Color::rgb(0.0, 0.0, 0.0);
        let mut cube_color = Color::rgb(1.0, 1.0, 1.0);
        for (i, entity) in children.iter_descendants(scene_entity).enumerate() {
            let name = match names.get(entity) {
                Ok(name) => format!("{name}"),
                Err(_) => "".to_string(),
            };

            // println!("found name: {:#?}", name);
            if let Ok(mesh_handle) = mesh_handles.get(entity) {
                if let Some(mesh) = meshes.get(mesh_handle) {
                    num_meshes_seen += 1;
                    // println!("mesh: {:#?}", mesh);
                    if let Ok(material_handle) = material_handles.get(entity) {
                        if let Some(material) = materials.get_mut(material_handle) {
                            material.alpha_mode = AlphaMode::Blend;
                            material.base_color.set_a(0.55);
                        }
                    }

                    if name == "Cube.052" {
                        commands.entity(entity).despawn();
                    }
                    // println!("NAMED MESH NODE: {:#?}", name);
                    // if name == "staff_mesh".to_string() {
                    //     commands.entity(entity).despawn();
                    // }
                }
            } else {
                // println!("non mesh containing entity ");
                if let Ok(global_transform) = global_transforms.get(entity) {
                    // transform.translation = Vec3::new(1.0, 0.0, 0.0);

                    // if name == "Adventurer_Body".to_string() {
                    //     commands.entity(entity).despawn();
                    // }
                    // println!("NAMED NON MESH NODE: {:#?}", name);
                    num_non_meshes_seen += 1;
                    // println!("NAMED NON MESH NODE: {:#?}", name);
                    let cube_handle = meshes.add(Cube::new(0.01).into());
                    // cube_color.set_r(cube_color.r() + 0.1);
                    cube_color.set_r(cube_color.r());
                    let cube_material_handle = materials.add(StandardMaterial {
                        base_color: cube_color.clone(),
                        ..default()
                    });

                    if let Ok(local_transform) = transforms.get_mut(scene_entity) {
                        // println!("scene transform: {:#?}", scene_transform);
                        // let added_transform = Transform::from_xyz(scene_transform.get_field("x"), y, z)
                        let mut entity_commands = commands.spawn(PbrBundle {
                            mesh: cube_handle.clone(),
                            material: cube_material_handle.clone(),
                            transform: local_transform.clone().into(),
                            ..Default::default()
                        });
                        entity_commands.set_parent(entity);
                    }
                    // println!("with transform: {:#?}", transform);
                }
            }
            // if let Ok(mut transform) = transforms.get_mut(entity) {
            //     let color = Color::hsl(
            //         ((1 as f32 * 2.345 + time.elapsed_seconds_wrapped()) * 100.0) % 360.0,
            //         1.0,
            //         0.5,
            //     );

            //     transform.field_at_mut = color;

            //     println!("transform: {:#?}", transform);
            //     // transform.rotate_x(40.0);
            // }
        }
        println!("num meshes seen: {num_meshes_seen}");
        println!("num non meshes seen: {num_non_meshes_seen}");
    }
}
