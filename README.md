# vh_reader
Work on reading character and map files from the game Valheim. Current progress includes templates for the 010 hex editor for both file types, as well as some rudimentary gui in Rust for editing the character files. 

![gif of the template in action](/screenshots/vh_reader.gif?raw=true "gif of the template in action")

# Templates
[link to folder](/templates/)
## Character Files
Characters are stored in .fch files, currently the file format is completely mapped out with most of the important property names/relationships worked out, I am still lacking some property names though. I also haven't yet made any list of "valid" input values for most of the inputs, e.g. item names, beard names, etc.


## Map Data Files
Map data is stored mostly within .db files with some information in lightweight .fwl files. I have the structure of this file mapped out pretty well, but I am still relying on some big assumptions. There are many properties I have not discerned yet. 


# GUI app
Last April I started making a [little GUI app](/vhr_gui/) in rust using what I learned about the files from making the 010 templates. 
![preview of GUI](/screenshots/wip_inventory.png?raw=true "preview of GUI")
