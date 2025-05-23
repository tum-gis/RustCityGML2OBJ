mod conversion_functions;
mod geometry_functions;
mod write_functions;
mod translation_module;

static INPUT_DIR: &'static str = "/home/thomas/CityGML2OBJTestfolder/CityGML_3_files/citygml3_tile_for_testing/694_5334__v3.gml";
static OUTPUT_DIR: &'static str = "/home/thomas/CityGML2OBJTestfolder/output";
static TBW : bool = false;

fn main() {
    println!("Input Directory: {}", INPUT_DIR);
    println!("Output Directory: {}", OUTPUT_DIR);
    let overall_reader = ecitygml_io::CitygmlReader::from_path(INPUT_DIR);

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
