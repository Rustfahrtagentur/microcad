use bevy::{color::palettes::tailwind::*, prelude::*};

mod slint_bevy_adapter;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let ground_matl = materials.add(Color::from(GRAY_300));

    // Ground
    commands.spawn((
        Mesh3d(
            meshes.add(
                Plane3d {
                    normal: Dir3::Z,
                    ..Default::default()
                }
                .mesh()
                .size(5.0, 5.0),
            ),
        ),
        MeshMaterial3d(ground_matl),
    ));

    // Light
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(-Vec3::new(-0.15, -0.05, 0.25), Vec3::Y),
    ));
}
