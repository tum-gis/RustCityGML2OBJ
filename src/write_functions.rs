use crate::Args;
use egml::model::base::Id;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use clap::Parser;

pub fn write_obj_file(
    input_points: Vec<[f64; 3]>,
    triangles: Vec<u32>,
    building_id: &Id,
    semantic_surface_id: &Id,
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
}