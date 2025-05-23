use crate::translation_module::process_building_components_sequential;
use earcut::Earcut;
use earcut::utils3d::project3d_to_2d;
use ecitygml_core::model::building::Building;
use egml::model::geometry::Polygon;
use egml::operations::geometry::Geometry;
use crate::TBW;

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

pub fn translate_points() {}

pub fn triangulate(
    input_polygon: &Polygon,
    dx: Option<f64>,
    dy: Option<f64>,
    dz: Option<f64>,
) -> (Vec<u32>, Vec<[f64; 3]>) {
    // Predefine vectors for the points
    let mut all_points: Vec<[f64; 3]> = Vec::new();

    // Exterior ring
    let exterior_ring = &input_polygon.exterior;
    let length_of_exterior_ring = exterior_ring.points().len();
    
    // Append all the points to the pre defined vector
    for point in exterior_ring.points() {
        all_points.push([point.x(), point.y(), point.z()]);
    }

    // Interior rings
    for interior_ring in &input_polygon.interior {
        for point in interior_ring.points() {
            all_points.push([point.x(), point.y(), point.z()]);
        }
    }
    
    // Perform the translation into a local coordinate system if necessary
    if TBW{
        if let (Some(dx), Some(dy), Some(dz)) = (dx, dy, dz) {
            for point in &mut all_points {
                point[0] -= dx;
                point[1] -= dy;
                point[2] -= dz;
            }
        }
    }

    // Project 3D to 2D
    let mut all_points_projected: Vec<[f64; 2]> = Vec::new();
    let _ = project3d_to_2d(
        &all_points,
        length_of_exterior_ring,
        &mut all_points_projected,
    );

    // Prepare hole indices only if there are any interior rings
    let mut hole_indices: Vec<u32> = Vec::new();
    if !input_polygon.interior.is_empty() {
        let mut offset = exterior_ring.points().len() as u32;
        for ring in &input_polygon.interior[..input_polygon.interior.len().saturating_sub(1)] {
            offset += ring.points().len() as u32;
            hole_indices.push(offset);
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

    return (triangles, all_points);
}
