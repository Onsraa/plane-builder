mod ui;

use crate::ui::plugin::UiPlugin;
use bevy::color::palettes::css::SILVER;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::mesh::VertexAttributeValues;

#[derive(Component)]
struct WavePlane;

#[derive(Resource)]
pub struct PlaneParameters {
    pub amplitude: f32,
    pub frequency: f32,
    pub phase: f32,
    pub subdivisions: u32,
    pub size: [f32; 2],
}

impl Default for PlaneParameters {
    fn default() -> Self {
        Self {
            amplitude: 0.2,
            frequency: 5.0,
            phase: 0.0,
            subdivisions: 150,
            size: [10., 10.]
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WireframePlugin)
        .add_plugins(UiPlugin)
        .init_resource::<PlaneParameters>()
        .add_systems(Startup, setup)
        .add_systems(Update, (toggle_wireframe, animate_plane))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 7., 20.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));

    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.,
            range: 100.0,
            shadow_depth_bias: 0.2,
            ..default()
        },
        Transform::from_xyz(8.0, 16.0, 8.0).looking_at(Vec3::new(0., 1., 0.), Vec3::Y),
    ));

    commands.spawn((
        WavePlane,
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0).subdivisions(150))),
        MeshMaterial3d(materials.add(Color::from(SILVER))),
    ));
}

fn animate_plane(
    time: Res<Time>,
    parameters: Res<PlaneParameters>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<&Mesh3d, With<WavePlane>>,
) {
    for mesh_handle in &query {
        if let Some(mesh) = meshes.get_mut(&mesh_handle.0) {
            if let Some(VertexAttributeValues::Float32x3(positions)) =
                mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
            {
                for vertex in positions.iter_mut() {
                    let x = vertex[0];
                    let z = vertex[2];

                    // Calcul de l'ondulation
                    let distance = (x * x + z * z).sqrt();
                    let time_factor = time.elapsed_secs() * parameters.frequency;

                    // Modification de la hauteur (y) selon une onde sinusoïdale
                    vertex[1] = parameters.amplitude
                        * (distance * parameters.frequency + time_factor + parameters.phase).sin();
                }
            }

            // Recalcul des normales pour un éclairage correct
            mesh.compute_normals();
        }
    }
}

fn toggle_wireframe(
    mut wireframe_config: ResMut<WireframeConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}
