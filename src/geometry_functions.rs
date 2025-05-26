use crate::translation_module::process_building_components_sequential;
use earcut::Earcut;
use earcut::utils3d::project3d_to_2d;
use ecitygml_core::model::building::Building;
use egml::model::geometry::Polygon;
use egml::operations::geometry::Geometry;
use egml::model::base::Id;

// This function is used to calculate the translation parameters for a single building
pub fn get_building_wise_translation_parameters(
    input_building: &Building,
) -> Option<(f64, f64, f64)> {
    let all_building_points = process_building_components_sequential(input_building);

    if all_building_points.is_empty() {
        return None;
    }

    let mut sum_x = 0.0;
    let mut sum_y = 0.0;
    let mut sum_z = 0.0;

    for point in &all_building_points {
        sum_x += point[0];
        sum_y += point[1];
        sum_z += point[2];
    }

    let count = all_building_points.len() as f64;
    Some((sum_x / count, sum_y / count, sum_z / count))
}

pub fn triangulate(input_polygon: &Polygon) -> (Vec<u32>, Vec<[f64; 3]>) {
    // Collect all 3D points
    let mut all_points: Vec<[f64; 3]> = Vec::new();

    // Exterior ring
    let exterior_ring = &input_polygon.exterior;
    for point in exterior_ring.points() {
        all_points.push([point.x(), point.y(), point.z()]);
    }

    // Interior rings
    for interior_ring in &input_polygon.interior {
        for point in interior_ring.points() {
            all_points.push([point.x(), point.y(), point.z()]);
        }
    }

    // Project to 2D
    let mut all_points_projected: Vec<[f64; 2]> = Vec::new();
    let _ = project3d_to_2d(
        &all_points,
        exterior_ring.points().len(),
        &mut all_points_projected,
    );

    // Build hole indices (start index of each hole ring in the flattened point list)
    let mut hole_indices: Vec<u32> = Vec::new();
    if !input_polygon.interior.is_empty() {
        let mut offset = exterior_ring.points().len() as u32;
        for ring in &input_polygon.interior {
            hole_indices.push(offset);
            offset += ring.points().len() as u32;
        }
    }

    // Perform triangulation
    let mut triangles = vec![];
    let mut earcut = Earcut::new();
    earcut.earcut(
        all_points_projected.iter().copied(),
        &hole_indices,
        &mut triangles,
    );

    (triangles, all_points)
}

pub fn write_json_file(building_id: &Id, component_id: &Id, crs_info: &str, translation_parameters: (f64, f64, f64)){
    
}
