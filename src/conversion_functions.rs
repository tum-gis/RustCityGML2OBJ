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
use rayon::prelude::*;

pub fn process_building_components(
    input_building: &mut Building,
    tbw: bool,
    addBB: bool,
    addJSON: bool,
) {
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

    // Wall surfaces
    let building_id_for_walls = building_id.clone();
    let all_wall_surface = &input_building.wall_surface;
    all_wall_surface.par_iter().for_each(|wall_surface| {
        process_wall_surface(
            wall_surface,
            &building_id_for_walls,
            addBB,
            addJSON,
            dx,
            dy,
            dz,
        );
    });

    // Roof surfaces
    let building_id_for_roofs = building_id.clone();
    let all_roof_surface = &input_building.roof_surface;
    all_roof_surface.par_iter().for_each(|roof_surface| {
        process_roof_surface(
            roof_surface,
            &building_id_for_roofs,
            addBB,
            addJSON,
            dx,
            dy,
            dz,
        );
    });

    // Ground surfaces
    let building_id_for_grounds = building_id.clone();
    let all_ground_surface = &input_building.ground_surface;
    all_ground_surface.par_iter().for_each(|ground_surface| {
        process_ground_surface(
            ground_surface,
            &building_id_for_grounds,
            addBB,
            addJSON,
            dx,
            dy,
            dz,
        );
    });
}

pub fn process_wall_surface(
    input_wall_surface: &WallSurface,
    building_id: &Id,
    addBB: bool,
    addJSON: bool,
    dx: f64,
    dy: f64,
    dz: f64,
) {
    let thematic_info = "WallSurface";
    // Consider the thematic surfaces
    let multi_surfaces = &input_wall_surface.thematic_surface.lod3_multi_surface;
    if let Some(multi_surface) = multi_surfaces {
        // get the id of the multi surface
        let multi_surfaces_id = &multi_surface.gml.id;
        process_multi_surface(
            &multi_surface,
            building_id,
            multi_surfaces_id,
            thematic_info,
            false,
            addBB,
            addJSON,
            dx,
            dy,
            dz,
        )
    }
    // Consider the window surfaces
    let window_surfaces = &input_wall_surface.window_surface;
    for window_surface in window_surfaces {
        process_window_surface(window_surface, building_id, addBB, addJSON, dx, dy, dz);
    }

    // Consider the door surfaces
    let door_surfaces = &input_wall_surface.door_surface;
    for door_surface in door_surfaces {
        process_door_surface(door_surface, building_id, addBB, addJSON, dx, dy, dz);
    }
}

pub fn process_window_surface(
    input_window_surface: &WindowSurface,
    building_id: &Id,
    addBB: bool,
    addJSON: bool,
    dx: f64,
    dy: f64,
    dz: f64,
) {
    let thematic_info = "WindowSurface";
    let occupied_space = &input_window_surface.occupied_space;
    //let window_id = &occupied_space.space.city_object.gml.id;
    let space = &occupied_space.space;
    for multi_surface in &space.lod3_multi_surface {
        let window_id = &multi_surface.gml.id;
        process_multi_surface(
            &multi_surface,
            building_id,
            &window_id,
            thematic_info,
            true,
            addBB,
            addJSON,
            dx,
            dy,
            dz,
        );
    }
}

pub fn process_door_surface(
    input_door_surface: &DoorSurface,
    building_id: &Id,
    addBB: bool,
    addJSON: bool,
    dx: f64,
    dy: f64,
    dz: f64,
) {
    let thematic_info = "DoorSurface";
    let occupied_space = &input_door_surface.occupied_space;
    //let window_id = &occupied_space.space.city_object.gml.id;
    let space = &occupied_space.space;
    for multi_surface in &space.lod3_multi_surface {
        let window_id = &multi_surface.gml.id;
        process_multi_surface(
            &multi_surface,
            building_id,
            &window_id,
            thematic_info,
            true,
            addBB,
            addJSON,
            dx,
            dy,
            dz,
        );
    }
}

pub fn process_roof_surface(
    input_roof_surface: &RoofSurface,
    building_id: &Id,
    addBB: bool,
    addJSON: bool,
    dx: f64,
    dy: f64,
    dz: f64,
) {
    let thematic_info = "RoofSurface";
    let multi_surfaces = &input_roof_surface.thematic_surface.lod3_multi_surface;
    if let Some(multi_surface) = multi_surfaces {
        // get the id of the multi surface
        let multi_surfaces_id = &multi_surface.gml.id;
        process_multi_surface(
            &multi_surface,
            building_id,
            multi_surfaces_id,
            thematic_info,
            false,
            addBB,
            addJSON,
            dx,
            dy,
            dz,
        )
    }
}

pub fn process_ground_surface(
    input_ground_surface: &GroundSurface,
    building_id: &Id,
    addBB: bool,
    addJSON: bool,
    dx: f64,
    dy: f64,
    dz: f64,
) {
    let thematic_info = "GroundSurface";
    let multi_surfaces = &input_ground_surface.thematic_surface.lod3_multi_surface;
    if let Some(multi_surface) = multi_surfaces {
        // get the id of the multi surface
        let multi_surfaces_id = &multi_surface.gml.id;
        process_multi_surface(
            &multi_surface,
            building_id,
            multi_surfaces_id,
            thematic_info,
            false,
            addBB,
            addJSON,
            dx,
            dy,
            dz,
        )
    }
}

pub fn process_multi_surface(
    input_multi_surface: &MultiSurface,
    building_id: &Id,
    multi_surface_id: &Id,
    thematic_info: &str,
    processing_windows: bool,
    addBB: bool,
    addJSON: bool,
    dx: f64,
    dy: f64,
    dz: f64,
) {
    let surface_members = input_multi_surface.surface_member();
    for surface_member in surface_members {
        process_surface_member(
            &surface_member,
            building_id,
            multi_surface_id,
            thematic_info,
            processing_windows,
            addBB,
            addJSON,
            dx,
            dy,
            dz,
        );
    }
}

pub fn process_surface_member(
    input_surface_member: &Polygon,
    building_id: &Id,
    multi_surface_id: &Id,
    thematic_info: &str,
    process_with_poly_id: bool,
    addBB: bool,
    addJSON: bool,
    dx: f64,
    dy: f64,
    dz: f64,
) {
    // Perform the triangulation.
    let (triangles, all_points) = triangulate(input_surface_member);
    let input_surface_member_id = &input_surface_member.gml.id;
    if process_with_poly_id {
        write_obj_file(
            all_points,
            triangles,
            building_id,
            input_surface_member_id,
            &thematic_info,
            dx,
            dy,
            dz,
        );
    } else {
        // todo: Write the results to obj-format
        write_obj_file(
            all_points,
            triangles,
            building_id,
            input_surface_member_id,
            &thematic_info,
            dx,
            dy,
            dz,
        );
    }

    if addBB {
        // todo: add functionality to add the bounding box
    }

    if addJSON {
        // todo: add functionality to add the json file
    }
}
