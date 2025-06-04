use crate::translation_module::process_building_components_sequential;
use earcut::Earcut;
use earcut::utils3d::project3d_to_2d;
use ecitygml_core::model::building::Building;
use ecitygml_core::operations::FeatureWithGeometry;
use egml::model::geometry::Polygon;
use egml::operations::geometry::Geometry;

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

pub fn construct_buffered_bounding_box(
    input_building: &Building,
) -> (Vec<[f64; 3]>, Vec<[u64; 3]>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Get the envelope
    if let Some(envelope) = input_building.envelope() {
        let lower = envelope.lower_corner();
        let upper = envelope.upper_corner();

        // Apply 1m buffer
        let x_min = lower.x() - 2.0;
        let y_min = lower.y() - 2.0;
        let z_min = lower.z() - 2.0;
        let x_max = upper.x() + 2.0;
        let y_max = upper.y() + 2.0;
        let z_max = upper.z() + 2.0;

        // Define the 8 corners of the buffered box
        let corners = vec![
            [x_min, y_min, z_min],
            [x_max, y_min, z_min],
            [x_max, y_max, z_min],
            [x_min, y_max, z_min],
            [x_min, y_min, z_max],
            [x_max, y_min, z_max],
            [x_max, y_max, z_max],
            [x_min, y_max, z_max],
        ];

        // Add corners to vertices list
        vertices.extend_from_slice(&corners);

        let base_index = vertices.len() as u64;

        // For each corner, add a small triangle reflector
        let mut next_index = base_index;

        let offset = 0.5; // size of the corner reflector

        for &corner in &corners {
            // Place the tip of the pyramid offset along all axes
            let tip = [
                corner[0] + offset * (if corner[0] == x_min { 1.0 } else { -1.0 }),
                corner[1] + offset * (if corner[1] == y_min { 1.0 } else { -1.0 }),
                corner[2] + offset * (if corner[2] == z_min { 1.0 } else { -1.0 }),
            ];

            // Add tip vertex
            vertices.push(tip);
            let tip_idx = next_index;
            next_index += 1;

            // Create triangle faces from tip to 3 adjacent box edges
            let corner_idx = vertices.iter().position(|&v| v == corner).unwrap() as u64;

            // Choose 3 neighbors on the bounding box that share the same corner
            // For simplicity, just add 3 edges from the corner to its neighbors in x, y, z
            let neighbors = [
                [corner[0], corner[1], tip[2]], // z neighbor
                [corner[0], tip[1], corner[2]], // y neighbor
                [tip[0], corner[1], corner[2]], // x neighbor
            ];

            let mut neighbor_indices = Vec::new();
            for neighbor in neighbors {
                let idx = vertices.iter().position(|&v| v == neighbor);
                let n_idx = if let Some(i) = idx {
                    i as u64
                } else {
                    vertices.push(neighbor);
                    let i = next_index;
                    next_index += 1;
                    i
                };
                neighbor_indices.push(n_idx);
            }

            // Triangles from tip to each of the 3 neighbor edges
            for &n_idx in &neighbor_indices {
                indices.push([tip_idx, corner_idx, n_idx]);
            }
        }

        // Done
        return (vertices, indices);
    }

    println!("Envelope hat keine gültigen lower/upper corner Koordinaten.");
    (vertices, indices)
}

pub fn import_bounding_box(path_to_bounding_box: &str){
    // todo: muss noch implementiert werden
}
