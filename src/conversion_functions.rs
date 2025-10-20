use crate::geometry_functions::{construct_buffered_bounding_box, triangulate};
use crate::write_functions::write_obj_file;
use ecitygml::operations::GeometryCollector;
use ecitygml_core::model::building::Building;
use ecitygml_core::model::common::{CityObjectClass, LevelOfDetail};
use ecitygml_core::operations::{FeatureWithGeometry, Visitable};
use egml::model::base::Id;
use egml::model::geometry::{MultiSurface, Polygon};
use rayon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use egml::operations::triangulate::Triangulate;

// Helper container that stores all vertices and triangles that belong to one
// semantic surface class (e.g. WallSurface, RoofSurface, …).
#[derive(Debug, Default)]
struct SurfaceGroup {
    vertices: Vec<[f64; 3]>,
    triangles: Vec<u32>,
}

pub fn collect_building_geometries(
    input_building: &mut Building,
    tbw: bool,
    add_bb: bool,
    add_json: bool,
    import_bb: bool,
    group_by_surface: bool,
) {
     // Initialize an empty bounding box
    let mut bbox = (Vec::new(), Vec::new());

    // Distinguish the different cases of the bounding box
    if add_bb {
        // Calculate the bounding box and add little pyramids in the corners
        bbox = construct_buffered_bounding_box(input_building);
    } else if import_bb {
        // Import the bounding box from an external file
        // todo: Muss noch implementiert werden.
    }

    //let building_id = &input_building.city_object.gml.id;
    let building_id = &input_building.occupied_space.space.city_object.gml.id;
    
    // get the translation parameter into a local crs in case it is desired
    let mut dx: f64 = 0.0;
    let mut dy: f64 = 0.0;
    let mut dz: f64 = 0.0;
    if tbw {
        // Calculate the envelope of the building and get the tranformation parameters from it
        if let Some(envelope) = input_building.envelope() {
            let upper_corner = envelope.upper_corner();
            let lower_corner = envelope.lower_corner();
            dx = -((upper_corner.x() + lower_corner.x()) / 2.0);
            dy = -((upper_corner.y() + lower_corner.y()) / 2.0);
            dz = -((upper_corner.z() + lower_corner.z()) / 2.0);
        } else {
            println!("Envelope hat keine gültigen lower/upper corner Koordinaten.");
        }
    }

    let mut collector_1 = GeometryCollector::new();
    input_building.accept(&mut collector_1);

    // Prepare optional shared accumulator when grouping option is invoked
    let groups: Option<Arc<Mutex<HashMap<String, SurfaceGroup>>>> = if group_by_surface {
        Some(Arc::new(Mutex::new(HashMap::new())))
    } else {
        None
    };

    // Assuming collector_1.city_objects is a Vec or similar iterable
    collector_1
        .city_objects
        .par_iter()
        .for_each(|collected_geometry| {
            // Obtain all the different data
            let gml_id = &collected_geometry.1.gml.id;
            let class = collected_geometry.1.class;
            let implicit_geometries = &collected_geometry.1.implicit_geometries;
            let multi_surfaces = &collected_geometry.1.multi_surfaces;
            let solids = &collected_geometry.1.solids;

            // The textual representation of the class (e.g. "WallSurface")
            let class_key = city_object_class_to_str(class).to_owned();
            
            // Process the MultiSurfaces
            for multi_surface in multi_surfaces {
                process_multi_surface(
                    &multi_surface,
                    building_id,
                    class,
                    dx,
                    dy,
                    dz,
                    &bbox,
                    gml_id,
                    groups.clone(),          // ← optional shared map
                    class_key.clone(),       // ← semantic key for grouping
                );
            }
        });
    if let Some(groups_arc) = groups {
        // No other thread holds the Arc at this point → we can unwrap it safely
        let map = Arc::try_unwrap(groups_arc)
            .expect("Unexpected Arc reference count")
            .into_inner()
            .unwrap();

        for (class_key, group) in map {
            let filename = format!("{}_{}.obj", building_id, class_key);

            // We reuse `write_obj_file` – it derives the final file name from the
            // supplied IDs, so we create a temporary `Id` that contains the
            // desired filename.
            write_obj_file(
                group.vertices,
                group.triangles,
                building_id,
                &Id::from_hashed_string(&filename),        // temporary ID used only for naming
                &class_key,         // thematic info (same as class name)
                dx,
                dy,
                dz,
                &bbox,
                &Id::from_hashed_string("grouped"), // dummy multi‑surface ID
                &Id::from_hashed_string("grouped"),
            );

            println!("✔️  Wrote grouped OBJ for {} → {}", class_key, filename);
        }
    }

    if add_json {
        // TODO: muss noch implementiert werden
    }
}

pub fn process_multi_surface(
    input_multi_surface: &(&LevelOfDetail, &MultiSurface),
    building_id: &Id,
    class: CityObjectClass,
    dx: f64,
    dy: f64,
    dz: f64,
    bbox: &(Vec<[f64; 3]>, Vec<[u64; 3]>),
    gml_id: &Id,
    groups: Option<Arc<Mutex<HashMap<String, SurfaceGroup>>>>, // ← NEW
    class_key: String,                                          // ← NEW
) {
    let stuffs = &input_multi_surface.1.surface_member();
    let stuff_gml_id = &input_multi_surface.1.gml.id;
    stuffs.par_iter().for_each(|surface_member| {
        process_surface_member(
            surface_member,
            building_id,
            stuff_gml_id,
            class,
            dx,
            dy,
            dz,
            bbox,
            gml_id,
            class_key.clone(),
            groups.clone(),
            
        );
    });
}
    

pub fn process_surface_member(
    input_surface_member: &Polygon,
    building_id: &Id,
    multi_surface_id: &Id,
    thematic_info: CityObjectClass,
    dx: f64,
    dy: f64,
    dz: f64,
    bbox: &(Vec<[f64; 3]>, Vec<[u64; 3]>),
    gml_id: &Id,
    class_key: String, // ← semantic class name (used as map key)
    groups: Option<Arc<Mutex<HashMap<String, SurfaceGroup>>>>, // ← optional map
) {
    let (triangles, all_points) = triangulate(input_surface_member);
    let surface_id = input_surface_member.gml.id.clone();

    if let Some(groups_arc) = groups {
        // -------------------------- GROUPED MODE ---------------------------
        let mut map = groups_arc.lock().unwrap();

        // Get (or create) the buffer that belongs to this semantic class
        let bucket = map.entry(class_key).or_default();

        // OBJ indices are 1‑based – keep a running offset for this bucket
        let vertex_offset = bucket.vertices.len() as u32;

        // Store the vertices
        bucket.vertices.extend_from_slice(&all_points);

        // Shift the triangle indices so they refer to the global vertex list
        // of this bucket and store them.
        let shifted: Vec<u32> = triangles
            .into_iter()
            .map(|idx| idx + vertex_offset)   // add the offset to every index
            .collect();
        bucket.triangles.extend(shifted);
    } else {
        // -------------------------- LEGACY MODE ---------------------------
        let thematic_info_string = city_object_class_to_str(thematic_info);
        write_obj_file(
            all_points,
            triangles,
            building_id,
            &surface_id,
            &thematic_info_string,
            dx,
            dy,
            dz,
            bbox,
            gml_id,
            multi_surface_id,
        );
    }
}

pub fn city_object_class_to_str(class: CityObjectClass) -> &'static str {
    match class {
        CityObjectClass::AuxiliaryTrafficArea => "AuxiliaryTrafficArea",
        CityObjectClass::AuxiliaryTrafficSpace => "AuxiliaryTrafficSpace",
        CityObjectClass::Bridge => "Bridge",
        CityObjectClass::BridgeConstructiveElement => "BridgeConstructiveElement",
        CityObjectClass::BridgeFurniture => "BridgeFurniture",
        CityObjectClass::BridgeInstallation => "BridgeInstallation",
        CityObjectClass::BridgePart => "BridgePart",
        CityObjectClass::BridgeRoom => "BridgeRoom",
        CityObjectClass::Building => "Building",
        CityObjectClass::BuildingConstructiveElement => "BuildingConstructiveElement",
        CityObjectClass::BuildingFurniture => "BuildingFurniture",
        CityObjectClass::BuildingInstallation => "BuildingInstallation",
        CityObjectClass::BuildingPart => "BuildingPart",
        CityObjectClass::BuildingRoom => "BuildingRoom",
        CityObjectClass::BuildingUnit => "BuildingUnit",
        CityObjectClass::CeilingSurface => "CeilingSurface",
        CityObjectClass::CityFurniture => "CityFurniture",
        CityObjectClass::CityObjectGroup => "CityObjectGroup",
        CityObjectClass::ClearanceSpace => "ClearanceSpace",
        CityObjectClass::Door => "Door",
        CityObjectClass::DoorSurface => "DoorSurface",
        CityObjectClass::FloorSurface => "FloorSurface",
        CityObjectClass::GenericLogicalSpace => "GenericLogicalSpace",
        CityObjectClass::GenericOccupiedSpace => "GenericOccupiedSpace",
        CityObjectClass::GenericThematicSurface => "GenericThematicSurface",
        CityObjectClass::GenericUnoccupiedSpace => "GenericUnoccupiedSpace",
        CityObjectClass::GroundSurface => "GroundSurface",
        CityObjectClass::Hole => "Hole",
        CityObjectClass::HoleSurface => "HoleSurface",
        CityObjectClass::HollowSpace => "HollowSpace",
        CityObjectClass::InteriorWallSurface => "InteriorWallSurface",
        CityObjectClass::Intersection => "Intersection",
        CityObjectClass::Marking => "Marking",
        CityObjectClass::OtherConstruction => "OtherConstruction",
        CityObjectClass::OuterCeilingSurface => "OuterCeilingSurface",
        CityObjectClass::OuterFloorSurface => "OuterFloorSurface",
        CityObjectClass::PlantCover => "PlantCover",
        CityObjectClass::Railway => "Railway",
        CityObjectClass::Road => "Road",
        CityObjectClass::RoofSurface => "RoofSurface",
        CityObjectClass::Section => "Section",
        CityObjectClass::SolitaryVegetationObject => "SolitaryVegetationObject",
        CityObjectClass::Square => "Square",
        CityObjectClass::Story => "Story",
        CityObjectClass::Track => "Track",
        CityObjectClass::TrafficArea => "TrafficArea",
        CityObjectClass::TrafficSpace => "TrafficSpace",
        CityObjectClass::Tunnel => "Tunnel",
        CityObjectClass::TunnelConstructiveElement => "TunnelConstructiveElement",
        CityObjectClass::TunnelFurniture => "TunnelFurniture",
        CityObjectClass::TunnelInstallation => "TunnelInstallation",
        CityObjectClass::TunnelPart => "TunnelPart",
        CityObjectClass::WallSurface => "WallSurface",
        CityObjectClass::WaterBody => "WaterBody",
        CityObjectClass::WaterGroundSurface => "WaterGroundSurface",
        CityObjectClass::WaterSurface => "WaterSurface",
        CityObjectClass::Waterway => "Waterway",
        CityObjectClass::Window => "Window",
        CityObjectClass::WindowSurface => "WindowSurface",
    }
}

