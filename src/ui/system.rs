use crate::{PlaneParameters, WavePlane};
use bevy::prelude::*;
use bevy_egui::*;

pub fn ui_system(
    mut contexts: EguiContexts,
    mut plane_parameters: ResMut<PlaneParameters>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut plane_query: Query<&Mesh3d, With<WavePlane>>,
) {
    let plane_handle = plane_query.single();

    egui::Window::new("Plane parameters").show(contexts.ctx_mut(), |ui| {
        ui.add_space(5.0);
        ui.add(egui::Slider::new(&mut plane_parameters.amplitude, 0.0..=5.0).text("Amplitude"));
        ui.add_space(5.0);
        ui.add(egui::Slider::new(&mut plane_parameters.frequency, 0.0..=10.0).text("Frequency"));
        ui.add_space(5.0);

        let mut subdivisions = plane_parameters.subdivisions as i32;
        let mut size = plane_parameters.size;
        let mut changed = false;

        changed |= ui.add(egui::Slider::new(&mut subdivisions, 1..=150).text("Subdivisions")).changed();
        changed |= ui.add(egui::Slider::new(&mut size[0], 1.0..=30.0).text("Width")).changed();
        changed |= ui.add(egui::Slider::new(&mut size[1], 1.0..=30.0).text("Height")).changed();

        if changed {
            plane_parameters.size[0] = size[0];
            plane_parameters.size[1] = size[1];
            plane_parameters.subdivisions = subdivisions as u32;
            if let Some(mesh) = meshes.get_mut(&plane_handle.0) {
                *mesh = Mesh::from(
                    Plane3d::default()
                        .mesh()
                        .size(size[0], size[1])
                        .subdivisions(plane_parameters.subdivisions),
                );
            }
        }
    });
}
