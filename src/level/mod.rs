use bevy::prelude::*;

pub struct PlanePlugin;
impl Plugin for PlanePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_plane);
    }
}

fn setup_plane(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(50.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
}
