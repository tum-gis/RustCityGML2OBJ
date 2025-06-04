# :cityscape: RustCityGML2OBJ :cityscape:
Command line converter of **CityGML (.gml)** to **OBJ (.obj)** files. This project is at an early stage and is currently being further developed.
## :arrow_forward: How to run?

  `--input  your-input-citygml-path-here` 
  
  `--output  your-output-obj-path-here` 

Please make sure to use the absolute paths to the respective directories.

### Detailed Project Description
+ Every building will be converted into a set of `.obj` files
+ Every polygon will be triangulated with the [earcut Rust-library](https://github.com/ciscorn/earcut-rs) and will be written into an individual `.obj`
+ Every `.obj` files adheres to the following naming convention: `<gml_id-of-the-building>__<gml_id_of the polygon>.obj`

### :wrench: Optional features

| Optional feature | specification |
| -------- | -------- |
| Building-wise translation into local CRS before the triangulation |`--tbw`|
| Adding small triangular structures indicating the building-wise, axis-aligned bounding box to each of the resulting `.obj` files. |`--add_bb`|
| For every `.obj` file, write out an additional `.json` file contaiing metadata such as gml_id, thematic role, and the translation parameters that were applied, in case a translation into a local CRS was performed before the triangulation |`--add_json`|


### Ongoing Developments
+ extending the functionality to more parts of the CityGML 3.0 data model
+ implementing a functionality to convert the entire building into one single `.obj` file.
+ imorting an external bounding box
  

### CityGML Requirements:

#### Mandatory:

+ CityGML 3.0 (In case you have an older CityGML version, you can e.g. use the [citygml-tools](https://github.com/citygml4j/citygml-tools) to upgrade your files)
+ Files must end with `.gml`, `.GML`, `.xml`, or `.XML`
+ Your files must be valid (e.g., free check with [CityDoctor](https://transfer.hft-stuttgart.de/gitlab/citydoctor/citydoctor2))
 
## Limitations

+ Only Buildings are supported.
+ CityGML 1.0 and 2.0 are not supported
+ Only RoofSurfaces, WallSurfaces, GroundSurfaces, WindowSurfaces and DoorSurfaces are currently suppported.
+ Building Installations are not supportes
+ Implicit geometry is not Supported


## :mailbox: Contact & Feedback

Feel free to open a discussion under Issues or write us an email

- [Thomas Froech](thomas.froech@tum.de)
