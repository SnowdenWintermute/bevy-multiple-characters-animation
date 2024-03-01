mod link_animations;
mod run_animations;
use crate::asset_loader::{AssetLoaderState, AssetPack};
use bevy::{gltf::Gltf, prelude::*, render::mesh::shape::Cube, utils::HashMap};

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
    Done,
}

fn spawn_characters(
    mut commands: Commands,
    asset_pack: Res<AssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
    mut next_state: ResMut<NextState<SpawnCharacterState>>,
) {
    if let Some(gltf) = assets_gltf.get(&asset_pack.0) {
        // println!("gltf: {:#?}", gltf);
        for node in gltf.named_nodes.iter() {
            println!("node: {:#?}", node.0);
        }

        commands.spawn((
            SceneBundle {
                scene: gltf.named_scenes["Scene"].clone(),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            PlayerCharacterName("Player 1".to_string()),
        ));
        next_state.set(SpawnCharacterState::Done);

        // commands.spawn((
        //     SceneBundle {
        //         scene: gltf.named_scenes["Scene"].clone(),
        //         transform: Transform::from_xyz(2.0, 0.0, 0.0),
        //         ..Default::default()
        //     },
        //     PlayerCharacterName("Player 2".to_string()),
        // ));

        let mut animations = HashMap::new();
        animations.insert(
            String::from("Death"),
            gltf.named_animations["Death"].clone(),
        );

        animations.insert(String::from("Idle"), gltf.named_animations["Idle"].clone());

        commands.insert_resource(Animations(animations));
    }
}

pub fn list_scene_tree(
    mut commands: Commands,
    scene: Query<Entity, With<PlayerCharacterName>>,
    children: Query<&Children>,
    time: Res<Time>,
    material_handles: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
    asset_pack: Res<AssetPack>,
    assets_gltf: Res<Assets<Gltf>>,
) {
    println!("LISTING");
    for scene_entity in &scene {
        for (i, entity) in children.iter_descendants(scene_entity).enumerate() {
            if let Ok(mesh_handle) = mesh_handles.get(entity) {
                if let Some(mesh) = meshes.get(mesh_handle) {
                    // println!("mesh: {:#?}", mesh);
                    if let Ok(material_handle) = material_handles.get(entity) {
                        if let Some(material) = materials.get_mut(material_handle) {
                            material.alpha_mode = AlphaMode::Blend;
                            material.base_color.set_a(0.45);
                            // commands.entity(entity).despawn();
                        }
                    }
                }
            } else {
                println!("non mesh containing entity ");
                if let Ok(global_transform) = global_transforms.get(entity) {
                    // transform.translation = Vec3::new(1.0, 0.0, 0.0);
                    let cube_handle = meshes.add(Cube::new(0.1).into());
                    let cube_material_handle = materials.add(StandardMaterial {
                        base_color: Color::rgb(0.8, 0.0, 0.0),
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

                        // commands.entity(entity).push_children();
                    }

                    if let Some(gltf) = assets_gltf.get(&asset_pack.0) {}
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
    }
    println!("scene: {:#?}", scene);
}
