mod conversion_functions;
mod geometry_functions;
mod translation_module;
mod write_functions;
use clap::Parser;

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
static TBW: bool = false;

fn main() {
    let args = Args::parse();
    println!("Input Directory: {}", args.input);
    println!("Output Directory: {}", args.output);
    let overall_reader = ecitygml_io::CitygmlReader::from_path(args.input.clone());

    match overall_reader.unwrap().finish() {
        Ok(data) => {
            // take care of the buildings
            let all_buildings = &data.building;
            for building in all_buildings {
                conversion_functions::process_building_components(&building)
            }
            // todo: this has to be augmented to other semantic objects besides buildings
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
