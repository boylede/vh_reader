So I did it again. 
I spent a weekend trying to reverse another valheim file format. 
This time it was the maps. This time it took all weekend and a weeknight to just get the enumeration step done. It took so long. Embarrassingly long. 

The enumeration step is what i’m calling the part where you are able to walk through the file and understand the structure of it all. You don’t know what each byte does, but you know the hierarchy and are able to predictably load up any file, regardless of the length, and read through each byte until you get to the end and stop, and know precisely when to stop, never expecting there to be additional bytes but finding them missing, never leaving additional un-noticed bytes at the end. 

This file format is slightly more complicated than the character file, but not by much. It is divided into four sections which i was able to identify by visual patterns (though I thought the first two sections were part of the same section at first)

The first section has a pretty obvious cadence of about 108 bytes. There are a bunch of high-entropy high-value bytes with some low-value bytes in between, so the ascii text version shows the pattern clearly. This is helped significantly by several of the values coincidentally being within the ascii range so there are some easy symbols to latch onto while working through it. 

The second section, which i thought was part of the first, has a cadence of 20 bytes (but i knew the first section had a variable cadence as I had done some analysis and found several distances, more on that later) so i just thought the entries were maybe sorted by length and the big ones (108 bytes) were at the beginning and the small ones (20 bytes) ended up at the end. I thought there’d be a bunch in the middle with other values. This section also wasn’t in my 000 input file (the world that had barely been loaded when i quit the game asap after selecting the world)

The third section is very visually distinct from the surrounding bytes, as it is almost exclusively 0xff or 0x00 bytes, or values close to those two. I believe this section is pairs of integers, almost always negative but if positive then close to 0. I think they are pairs because it is prepended with a number that counts the pairs to follow, so that makes sense. 

The last section is the most obvious, and is basically the same as the POI’s from the character file. It is obvious because it is ascii text followed by some floats. Here we have the map’s own pois but it is weird because modifying these don’t appear to affect the map thus far. I have attempted to see what it would do and i did not find anything change when i altered these position values.

The first section took me the longest to figure out, and i needed to get past it to reliably understand the next sections. I don’t have a good reason why it took so long, i just couldn’t figure out the length for ages. I knew it was variable length, because fixed lengths didn’t work out for more than the first 40 or 50 entries. Sometimes i’d be able to get like 400 entries working with various different offsets, sizes, and length bytes. Finally i stumbled on the apparently correct solution. 

One of the techniques that i used was to look at the values for repeating patterns. I noticed that there was a four-byte value that repeated predictably in the input. I wasn’t sure what it was, and i’m still not, but at first i thought it might be a sigil to mark the beginning of a variable-sized entry. This is a rubbish way to serialize, but its not unheard of. I wrote a little analysis function to check on the sizes of these sigil placements and it confirmed by previous sizes, and showed several areas where the sizes changed. Each 108 byte structure sometimes has one of these sigils inside it so it definitely confused my analysis. Eventually i was able to find an int that controls the length of the rest of the entity and was able to set this aside while i worked out the rest. 

With that wrapped up, i was able to quickly write up the middle two sections based on some assumptions about the data types (assuming floats within reasonable maginitide ranges and amount of precision are floats, anything else is an int unless theres the wrong number of bytes and then find some chars thrown around) this has worked out OK for the character file. This file though has a lot more unknowns so it is less meaningful to just say “i bet this is a float” and be done. In this case the entities (i’m calling the first data structure that) may have different values depending on the actual entity type, but i havent identified that yet.

Having the enumeration done, i dived into making changes to the various things in the file to see how the game reacted. Changing random entity positions didn’t appear to do anything ( i tried to move stuff to where i was and nothing new showed up there). But when i went to an entity position (it turned out to be a rock) and hit it the entity did change, though i didn’t figure out what the changes where for that one. Then i repeated this with a tree (again, choosing a random entity and going to that location) and i was able to identify the “health” or “hitpoints” that some objects have. 

I was also able to decide that the rotations are stored in quaternions. I am not 100% sure but i tweaked the values of the rotation and it appeared to match this expectation, plus or minus since i didn’t spend too long on it. Its very possible that i have the values in the wrong order though as these are not necessarily standardized like xzy are. 


# Inside an entity

So entities within Valheim are actually pretty interesting, in terms of how they are serialized. Given the simplicity of the character file format, I was not expecting what I found here. Each entity has a number of fixed properties, such as position and rotation, and then can have any number of other properties added in containers at the end of the struct. In terms of "struct of arrays vs array of structs" it is a little bit of both. I assume that the game expects each entity type to have certain properties on it, but I haven't worked out if it is possible to add properties that were not expected. So far in tests I've done the game is ignoring unexpected properties. 

# What is in each map file

The map file keeps track of all the entities that have been spawned into the world, as you'd expect, as well as a list of deleted entities and a list of loaded chunks. There is also the list of "keys" that the map has unlocked, which I am assuming is basically the boss kills, and also a list of locations where specified structures are expected to be generated. 

# Chunks

So chunks appear to be 64x64 areas of the map which are centered around the middle point, so e.g. chunk (0,0) stretches from -32 to + 32 on each axis, save y which appears to be the full height of the map. 

# Structures

The map stores a list of Points-of-interest and, I think, a boolean of whether or not they have been seen (or maybe generated). The game deletes entries it decides not to use, based on one map file I observed atleast. 

I wasn't able to "trick" the game into generating a structure in a different location by changing these values, so these POI's are not simply the position of assets in the game world, they are more records of where that thing is. The actual position, I take it, is stored within each of the individual generated entities.

# Terrain

I have heard previously that the terrain modifications are stored as entities just like everything else, and this appears to be true. I found a terrain modification entity with several float properties attached that appeared to keep track of the absolute height of certain spots within the chunk.

# Chests, Portals, Boss Trophies, Tombstones, and Rocks

Several entity types contain string properties, and sometimes these string properties are actually base64 encoded data structures. Chests contain an array of Item structures, just like an Inventory from a player character file. Breakable environment pieces (rocks) contain an array of floats that (i think) determine the hit points of each child part. Interestingly, these appear to be fixed-length, so e.g. all the big rocks have 130 parts and all the small rocks have 5 parts. When a part's HP dips below 0 the negative value is stored in the array so the array length remains constant. Small rocks appear to have 15 hp per part and big rocks appear to have 50 per part. 

Tombstones are the most interesting, because they contain two strings, one for the inventory structure (base64) and one that is just the player's name (so it can be shown floating above). Signs and portals also have the respective text stored here, as you'd expect, but what I did not expect was to find strings inside a smelter's entity. Looks like the object name for anything inside a smelter is stored, individually, as strings. so if you put two copper into a smelter, it will have two string properties "CopperOre" etc. I assume the same is true for charcoal kilns, and I will check on the cooking stand to see if it also has a timer float.
