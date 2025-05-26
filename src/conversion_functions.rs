use crate::geometry_functions::triangulate;
use crate::write_functions::write_obj_file;
pub use ecitygml::model::building::Building;
pub use ecitygml_core::model::construction::{
    DoorSurface, GroundSurface, RoofSurface, WallSurface, WindowSurface,
};
use ecitygml_core::operations::FeatureWithGeometry;
use egml::model::base::Id;
use egml::model::geometry::{MultiSurface, Polygon};
use nalgebra::UnitQuaternion;
use nalgebra::base::Vector3;
use nalgebra::geometry::Isometry3;

pub fn process_building_components(input_building: &mut Building, tbw: bool) {
    // get the translation parameter into a local crs in case it is desired
    let mut dx: f64 = 0.0;
    let mut dy: f64 = 0.0;
    let mut dz: f64 = 0.0;
    let envelope1 = &input_building.envelope();
    if tbw {
        // Calculate the envelope of the building and get the tranformation parameters from it
        if let Some(envelope) = input_building.envelope() {
            let upper_corner = envelope.upper_corner();
            let lower_corner = envelope.lower_corner();
            dx = -((upper_corner.x() + lower_corner.x()) / 2.0);
            dy = -((upper_corner.y() + lower_corner.y()) / 2.0);
            dz = -((upper_corner.z() + lower_corner.z()) / 2.0);
            //println!("dx: {}, dy: {}, dz: {}", dx, dy, dz);
        } else {
            println!("Envelope hat keine gültigen lower/upper corner Koordinaten.");
        }
    }

    let translation_vector: Vector3<f64> = Vector3::new(dx, dy, dz);
    let translation_isometry =
        Isometry3::from_parts(translation_vector.into(), UnitQuaternion::identity());
    &input_building.apply_transform(&translation_isometry);

    // Obtain the building id
    let building_id = &input_building.city_object.gml.id;

    // Take care of the wall surfaces, first
    let all_wall_surface = &input_building.wall_surface;
    for wall_surface in all_wall_surface {
        // get the wall surface id
        process_wall_surface(wall_surface, building_id);
    }

    // Take care of the Roof Surfaces
    let all_roof_surface = &input_building.roof_surface;
    for roof_surface in all_roof_surface {
        process_roof_surface(roof_surface, building_id);
    }

    // Take care of the Ground Surfaces
    let all_ground_surface = &input_building.ground_surface;
    for ground_surface in all_ground_surface {
        process_ground_surface(ground_surface, building_id);
    }
}

pub fn process_wall_surface(input_wall_surface: &WallSurface, building_id: &Id) {
    // Consider the thematic surfaces
    let multi_surfaces = &input_wall_surface.thematic_surface.lod3_multi_surface;
    if let Some(multi_surface) = multi_surfaces {
        // get the id of the multi surface
        let multi_surfaces_id = &multi_surface.gml.id;
        process_multi_surface(&multi_surface, building_id, multi_surfaces_id)
    }
    // Consider the window surfaces
    let window_surfaces = &input_wall_surface.window_surface;
    for window_surface in window_surfaces {
        process_window_surface(window_surface, building_id);
    }

    // Consider the door surfaces
    let door_surfaces = &input_wall_surface.door_surface;
    for door_surface in door_surfaces {
        process_door_surface(door_surface, building_id);
    }
}

pub fn process_window_surface(input_window_surface: &WindowSurface, building_id: &Id) {
    let occupied_space = &input_window_surface.occupied_space;
    //let window_id = &occupied_space.space.city_object.gml.id;
    let space = &occupied_space.space;
    for multi_surface in &space.lod3_multi_surface {
        let window_id = &multi_surface.gml.id;  
        process_multi_surface(&multi_surface, building_id, &window_id);
    }
}

pub fn process_door_surface(input_window_surface: &DoorSurface, building_id: &Id) {
    // todo: Muss noch implementiert werden
    // ecitygml does right now not support window surfaces
}

pub fn process_roof_surface(input_roof_surface: &RoofSurface, building_id: &Id) {
    let multi_surfaces = &input_roof_surface.thematic_surface.lod3_multi_surface;
    if let Some(multi_surface) = multi_surfaces {
        // get the id of the multi surface
        let multi_surfaces_id = &multi_surface.gml.id;
        process_multi_surface(&multi_surface, building_id, multi_surfaces_id)
    }
}

pub fn process_ground_surface(input_ground_surface: &GroundSurface, building_id: &Id) {
    let multi_surfaces = &input_ground_surface.thematic_surface.lod3_multi_surface;
    if let Some(multi_surface) = multi_surfaces {
        // get the id of the multi surface
        let multi_surfaces_id = &multi_surface.gml.id;
        process_multi_surface(&multi_surface, building_id, multi_surfaces_id)
    }
}

pub fn process_multi_surface(
    input_multi_surface: &MultiSurface,
    building_id: &Id,
    multi_surface_id: &Id,
) {
    let surface_members = input_multi_surface.surface_member();
    for surface_member in surface_members {
        process_surface_member(&surface_member, building_id, multi_surface_id);
    }
}

pub fn process_surface_member(
    input_surface_member: &Polygon,
    building_id: &Id,
    multi_surface_id: &Id,
) {
    // Perform the triangulation.
    let (triangles, all_points) = triangulate(input_surface_member);

    // Write the results to obj-format
    write_obj_file(all_points, triangles, building_id, multi_surface_id);
}
