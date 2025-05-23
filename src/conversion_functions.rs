use crate::TBW;
use crate::geometry_functions::{get_building_wise_translation_parameters, translate_points, triangulate};
use crate::write_functions::write_obj_file;
pub use ecitygml::model::building::Building;
pub use ecitygml_core::model::construction::{GroundSurface, RoofSurface, WallSurface};
use egml::model::base::Id;
use egml::model::geometry::{MultiSurface, Polygon};

pub fn process_building_components(input_building: &Building) {
    // get the translation parameter into a local crs in case it is desired
    let mut dx: Option<f64> = None;
    let mut dy: Option<f64> = None;
    let mut dz: Option<f64> = None;

    if TBW {
        if let Some((x, y, z)) = get_building_wise_translation_parameters(&input_building) {
            dx = Some(x);
            dy = Some(y);
            dz = Some(z);
        } else {
            eprintln!("Translation parameters could not be retrieved.");
        }
    }
    // Obtain the building id
    let building_id = &input_building.city_object.gml.id;
    // Take care of the wall surfaces, first
    let all_wall_surface = &input_building.wall_surface;
    for wall_surface in all_wall_surface {
        // get the wall surface id
        process_wall_surface(wall_surface, building_id, dx, dy, dz);
    }

    // Take care of the Roof Surfaces
    let all_roof_surface = &input_building.roof_surface;
    for roof_surface in all_roof_surface {
        process_roof_surface(roof_surface, building_id, dx, dy, dz);
    }

    // Take care of the Ground Surfaces
    let all_ground_surface = &input_building.ground_surface;
    for ground_surface in all_ground_surface {
        process_ground_surface(ground_surface, building_id, dx, dy, dz);
    }
}

pub fn process_wall_surface(
    input_wall_surface: &WallSurface,
    building_id: &Id,
    dx: Option<f64>,
    dy: Option<f64>,
    dz: Option<f64>,
) {
    let multi_surfaces = &input_wall_surface.thematic_surface.lod2_multi_surface;
    if let Some(multi_surface) = multi_surfaces {
        // get the id of the multi surface
        let multi_surfaces_id = &multi_surface.gml.id;
        process_multi_surface(&multi_surface, building_id, multi_surfaces_id, dx, dy, dz)
    }
}

pub fn process_roof_surface(
    input_roof_surface: &RoofSurface,
    building_id: &Id,
    dx: Option<f64>,
    dy: Option<f64>,
    dz: Option<f64>,
) {
    let multi_surfaces = &input_roof_surface.thematic_surface.lod2_multi_surface;
    if let Some(multi_surface) = multi_surfaces {
        // get the id of the multi surface
        let multi_surfaces_id = &multi_surface.gml.id;
        process_multi_surface(&multi_surface, building_id, multi_surfaces_id, dx, dy, dz)
    }
}

pub fn process_ground_surface(
    input_ground_surface: &GroundSurface,
    building_id: &Id,
    dx: Option<f64>,
    dy: Option<f64>,
    dz: Option<f64>,
) {
    let multi_surfaces = &input_ground_surface.thematic_surface.lod2_multi_surface;
    if let Some(multi_surface) = multi_surfaces {
        // get the id of the multi surface
        let multi_surfaces_id = &multi_surface.gml.id;
        process_multi_surface(&multi_surface, building_id, multi_surfaces_id, dx, dy, dz)
    }
}

pub fn process_multi_surface(
    input_multi_surface: &MultiSurface,
    building_id: &Id,
    multi_surface_id: &Id,
    dx: Option<f64>,
    dy: Option<f64>,
    dz: Option<f64>,
) {
    let surface_members = input_multi_surface.surface_member();
    for surface_member in surface_members {
        process_surface_member(&surface_member, building_id, multi_surface_id, dx, dy, dz);
    }
}

pub fn process_surface_member(
    input_surface_member: &Polygon,
    building_id: &Id,
    multi_surface_id: &Id,
    dx: Option<f64>,
    dy: Option<f64>,
    dz: Option<f64>,
) {
       
    // Perform the triangulation.
    let (triangles, all_points) = triangulate(input_surface_member, dx, dy, dz);

    // Write the results to obj-format
    write_obj_file(all_points, triangles, building_id, multi_surface_id);
}
