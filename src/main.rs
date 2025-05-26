mod conversion_functions;
mod geometry_functions;
mod translation_module;
mod write_functions;
use clap::Parser;
use rayon::prelude::*; 

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file path
    #[arg(short, long)]
    input: String,

    /// Output directory
    #[arg(short, long)]
    output: String,

    /// Some boolean flag (like TBW)
    #[arg(long, default_value_t = false)]
    tbw: bool,
}


//static INPUT_DIR: &'static str =
//    "/home/thomas/CityGML2OBJTestfolder/CityGML_3_files/citygml3_tile_for_testing/694_5334__v3.gml";
//static OUTPUT_DIR: &'static str = "/home/thomas/CityGML2OBJTestfolder/output";

fn main() {
    let args = Args::parse();
    println!("Input Directory: {}", args.input);
    println!("Output Directory: {}", args.output);
    let overall_reader = ecitygml_io::CitygmlReader::from_path(args.input.clone());

    match overall_reader.unwrap().finish() {
        Ok(mut data) => {
            let all_buildings = &mut data.building;

            all_buildings
                .par_iter_mut()
                .for_each(|building| {
                    conversion_functions::process_building_components(building, args.tbw);
                });
        }
        Err(e) => {
            eprintln!("Error reading data: {:?}", e);
        }
    }
}
