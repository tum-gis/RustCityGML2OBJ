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

    /// Option for adding the bounding box to the obj files
    #[arg(long, default_value_t = false)]
    add_json: bool,

    /// Some boolean flag (like TBW)
    #[arg(long, default_value_t = false)]
    add_bb: bool,
}

//static INPUT_DIR: &'static str =
//    "/home/thomas/CityGML2OBJTestfolder/CityGML_3_files/citygml3_tile_for_testing/694_5334__v3.gml";
//static OUTPUT_DIR: &'static str = "/home/thomas/CityGML2OBJTestfolder/output";

fn main() {
    let args = Args::parse();
    println!("Input Directory: {}", args.input);
    println!("Output Directory: {}", args.output);

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
                            conversion_functions::process_building_components(building, args.tbw, args.add_bb, args.add_json);
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
