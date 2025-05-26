# :cityscape: RustCityGML2OBJ :cityscape:
Command line converter of **CityGML (.gml)** to **OBJ (.obj)** files, while maintaining the semantics 

## :arrow_forward: How to run?
The `CityGML2OBJs.py` represents the starting point of the code, choose this file when configuring the runtime and pass the following parameters:

  `--input  your-input-citygml-path-here` 
  
  `--output  your-output-obj-path-here` 

Please make sure to use the absolute paths to the respective directories.

### :wrench: Optional features

| Optional feature | specification |
| -------- | -------- |
| Building-wise translation into local CRS |`--tbw`|


### CityGML Requirements:

#### Mandatory:

+ CityGML 3.0
+ Files must end with `.gml`, `.GML`, `.xml`, or `.XML`
+ Your files must be valid (e.g., free check with [CityDoctor](https://transfer.hft-stuttgart.de/gitlab/citydoctor/citydoctor2)
 
## Limitations

+ CityGML 1.0 and 2.0 are not supported
+ Building Furniture is not supported
+ 

## :mailbox: Contact & Feedback

Feel free to open a discussion under Issues or write us an email

- [Thomas Froech](thomas.froech@tum.de)
