use crate::Args;
use clap::Parser;
use egml::model::base::Id;
use serde::Serialize;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

#[derive(Serialize)]
struct Metadata {
    building_id: String,
    semantic_surface_id: String,
    thematic_role: String,
    dx: String,
    dy: String,
    dz: String,
}

pub fn write_json_metadata(
    building_id: &Id,
    semantic_surface_id: &Id,
    thematic_role: &str,
    output_dir: &str,
    dx: f64,
    dy: f64,
    dz: f64,
) {
    let metadata = Metadata {
        building_id: building_id.to_string(),
        semantic_surface_id: semantic_surface_id.to_string(),
        thematic_role: thematic_role.to_string(),
        dx: dx.to_string(),
        dy: dy.to_string(),
        dz: dz.to_string(),
    };

    let filename = format!(
        "{}___{}.json",
        metadata.building_id, metadata.semantic_surface_id
    );

    let file_path = Path::new(output_dir).join(filename);

    let file = match File::create(&file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create JSON file: {}", e);
            return;
        }
    };

    let writer = BufWriter::new(file);
    if let Err(e) = serde_json::to_writer_pretty(writer, &metadata) {
        eprintln!("Failed to write JSON metadata: {}", e);
    }
}

pub fn import_bbox_from_file() {
    // todo: muss noch implementiert werden
}

pub fn write_obj_file(
    input_points: Vec<[f64; 3]>,
    triangles: Vec<u32>,
    building_id: &Id,
    semantic_surface_id: &Id,
    thematic_role: &str,
    dx: f64,
    dy: f64,
    dz: f64,
) {
    let args = Args::parse();
    let building_id_string = building_id.to_string();
    let semantic_surface_string = semantic_surface_id.to_string();
    let filename = format!("{}___{}.obj", building_id_string, semantic_surface_string);

    let file_path = Path::new(&args.output).join(filename);

    let file = match File::create(&file_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create file: {}", e);
            return;
        }
    };

    let mut writer = BufWriter::new(file);

    // Write vertices
    for point in input_points.iter() {
        if let Err(e) = writeln!(writer, "v {} {} {}", point[0], point[1], point[2]) {
            eprintln!("Failed to write vertex: {}", e);
            return;
        }
    }

    // Write faces (triangles)
    if triangles.len() % 3 != 0 {
        eprintln!("Triangle index list is not a multiple of 3.");
        return;
    }

    for face in triangles.chunks(3) {
        if face.len() == 3 {
            // OBJ format uses 1-based indexing
            if let Err(e) = writeln!(writer, "f {} {} {}", face[0] + 1, face[1] + 1, face[2] + 1) {
                eprintln!("Failed to write face: {}", e);
                return;
            }
        }
    }

    if args.add_json {
        // write out the json file containing the metadata
        write_json_metadata(
            building_id,
            semantic_surface_id,
            thematic_role,
            &args.output,
            dx,
            dy,
            dz,
        );
    }
}
