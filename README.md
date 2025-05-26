# :cityscape: RustCityGML2OBJ :cityscape:
Command line converter of **CityGML (.gml)** to **OBJ (.obj)** files, while maintaining the semantics 

## :arrow_forward: How to run?

  `--input  your-input-citygml-path-here` 
  
  `--output  your-output-obj-path-here` 

Please make sure to use the absolute paths to the respective directories.

### :wrench: Detailed Project Description
+ Every building will be converted into a set of `.obj` files
+ Every polygon will be triangulated with the [earcut Rust-library](https://github.com/ciscorn/earcut-rs) and will be written into an individual `.obj`
+ Every `.obj` files adheres to the following naming convention: `<gml_id-of-the-building>__<gml_id_of the polygon>.obj`

### :wrench: Optional features

| Optional feature | specification |
| -------- | -------- |
| Building-wise translation into local CRS before the triangulation |`--tbw`|


### CityGML Requirements:

#### Mandatory:

+ CityGML 3.0
+ Files must end with `.gml`, `.GML`, `.xml`, or `.XML`
+ Your files must be valid (e.g., free check with [CityDoctor](https://transfer.hft-stuttgart.de/gitlab/citydoctor/citydoctor2)
 
## Limitations

+ Only Buildings are supported
+ CityGML 1.0 and 2.0 are not supported
+ Only RoofSurfaces, WallSurfaces, GroundSurfaces, WindowSurfaces and DoorSurfaces are currently suppported
+ Implicit geometry is not Supported


## :mailbox: Contact & Feedback

Feel free to open a discussion under Issues or write us an email

- [Thomas Froech](thomas.froech@tum.de)
