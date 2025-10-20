mod conversion_functions;
mod geometry_functions;
mod translation_module;
mod write_functions;

use clap::Parser;
use rayon::prelude::*;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file path
    #[arg(short, long)]
    input: String,

    /// Output directory
    #[arg(short, long)]
    output: String,

    /// Option for invoking building-wise translation into local CRS
    #[arg(long, default_value_t = false)]
    tbw: bool,

    /// Option for additionally writing out a json file containing metadata
    #[arg(long, default_value_t = false)]
    add_json: bool,

    /// Option for adding the bounding box to the obj files
    #[arg(long, default_value_t = false)]
    add_bb: bool,

    /// Option for importing a bounding box instead of creating a new one from the data
    #[arg(long, default_value_t = false)]
    import_bb: bool,
    
    /// Option for grouping the polygons by semantic surfaces
    #[arg(long, default_value_t = false)]
    group_sc: bool,
}

fn main() {
    let args = Args::parse();
    println!("Input Directory: {}", args.input);
    println!("Output Directory: {}", args.output);
    println!("TBW: {}", args.tbw);
    println!("Add: {}", args.add_json);
    println!("Import: {}", args.import_bb);
    println!("Group Sc: {}", args.group_sc);

    // Read directory entries
    let input_path = Path::new(&args.input);
    let entries = fs::read_dir(input_path).expect("Could not read input directory");

    // Filter and process each matching file
    for entry in entries.flatten() {
        let path = entry.path();

        // Check if the file ends with a valid extension
        if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
            let ext = ext.to_lowercase();
            if ext == "gml" || ext == "xml" {
                println!("Processing file: {}", path.display());

                let reader_result = ecitygml_io::CitygmlReader::from_path(&path);

                match reader_result.unwrap().finish() {
                    Ok(mut data) => {
                        let all_buildings = &mut data.building;

                        all_buildings.par_iter_mut().for_each(|building| {
                            conversion_functions::collect_building_geometries(
                                building,
                                args.tbw,
                                args.add_bb,
                                args.add_json,
                                args.import_bb,
                                args.group_sc,
                            );
                        });
                    }
                    Err(e) => {
                        eprintln!("Error reading file {}: {:?}", path.display(), e);
                    }
                }
            }
        }
    }
}
