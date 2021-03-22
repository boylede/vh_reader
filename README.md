# vh_reader
 Work on reading savefiles from the game Valheim. Current progress is a template for the 010 hex editor but once I understand what is stored in each file I'd like to work on some utilities for editing.

![Screenshot of template on a character file](/screenshots/screenshot.png?raw=true "Screenshot of template on a character file")

# What is this
I wanted to share a simple project I worked on this past weekend, and maybe it will grow into something more in the future. I’ve always enjoyed file formats for some reason so I had decided to open the savegame format used in Valheim and see if I could figure it out. I didn't look too hard online to see if someone else has done this, or if it is a common Unity format. Maybe my google-fu isn't very good but I never find much when looking up file formats even for formats well understood by the modding community.

I also wanted to make a "template" in the 010 hex editor, which I have read about before but I've never made a template for a file format with it. So here is my mental process of working through the Valheim savegame file and reversing it while making a template that works for any file in this format.

# Motivation
I wanted to take a look at these files and see what is in them, and get a better understanding of how the game works and how it thinks about the various entities within. What better way to do that then go directly to the source and see what data it is writing. Reverse engineering the entire game would likely be too big a task for me but looking at some savegame files should be more realistic, especially if they are not intentionally obfuscated too much.

# First look
In this case, the file format turns out to be relatively simple and easy to read. Not too much foreknowledge is needed and the game is currently very permissive in what it will accept from the file. Currently it doesn’t even deny files with invalid crc checks, which is really good for me because trying to figure out what specific algorithm, parameters, and range of input data used might have been so time consuming I might have lost motivation early.

Scrolling through a file for the first time I immediately see there are three sections with different data cadence and byte value ranges. 

![todo: add alt text](/screenshots/1.png?raw=true "todo: add title text")

The first section is exactly 100 bytes and appears to have two regions, one with some relatively low-value bytes spaced out with zeros followed by a higher-entropy region.

![todo: add alt text](/screenshots/2.png?raw=true "todo: add title text")

The second section is very large (most of the 4mb file) and is mostly 0x00 with an occasional grouping of 0x01 bytes. This is very difficult to scroll through in a hex editor as you can easily miss any data hiding in the middle of it so it is important to figure out how this section is layed out to avoid skipping sections. Indeed, in my first test file (my actual character i’ve been playing on) there are several islands of data in the middle i never saw until I found the size of the regions.

![todo: add alt text](/screenshots/3.png?raw=true "todo: add title text")

The third section at the end of the file was a relief to see. ASCII text values. This should make figuring out the meaning of each area way easier. Immediately I see my character name, the names of objects in my inventory, and a long long list of names of different items that exist in the game which I assume store everything the player has seen, since this is used when revealing crafting recipes to the player and other times when something first occurs such as the first time a player enters a biome. 

# Diving in - to a dead end.

Even though the third section contains the most readable and interesting data, I went back to the beginning to try and work out how the game figures out where to find this section. In order to make the template correctly identify the section, I need to know where it is. I could hard-code a value in as a placeholder and come back but I wanted to see how if I can figure out how to do it for any input file, not just my test file. Presumably when the game loads the file it doesn’t know where to find this data until it reads over the data before it, and I must do the same thing. Either the offset is stored in the file or baked into the loading procedure. 

I made a mental note of the location of the (apparent) beginning of the second and third sections I described above, to look for values of the same magnitude in the header. 

![todo: add alt text](/screenshots/4.png?raw=true "todo: add title text")

The third section appears to start at byte 0xC0112D, so I thought there might be some value in the header that pointed to this.  Looking at the very first bytes in the file I expected to find some kind of magic number to identify the file format, but this does not appear to exist. After looking at this for a while and not finding anything, I noticed that the first four bytes in the file could be read as an address that, while it is higher than the address I was expecting, is in the same magnitude range. The bytes in my example file are 0xC03590, which is 9,315 bytes past my supposed “third” section. 

![todo: add alt text](/screenshots/5.png?raw=true "todo: add title text")

In my example file this address was the first of four 0x00 bytes. Not very informative, but following that was the number 64 (0x40) in little-endian int32 format, and it was followed by 64 bytes of high-entropy data. Later, I decided that this must be a CRC of some kind. I later realized that the reason there were four 0’s was because I was actually looking at the end of the previous data structure which coincidentally had a zero value there, and the offset address is meant to be read as an offset after the first 4 bytes in the file. 

This was somewhat of a dead end, as the values in what I now assume are a CRC obviously do not point me towards anything else in the file, nor directly represent anything I could decode. However, I had at least figured out, to some degree, that the first four bytes in the file could be ignored for now and that the data structure at the very end of the file isn’t a table of contents or anything immediately useful. 


# Maps, and how to ignore them
At this point I didn’t have much, and I was relying on my intuition for that. Not a good position to start in, but I’m still having fun so may as well continue. 

![todo: add alt text](/screenshots/6.png?raw=true "todo: add title text")

I believed that the large field of zeros I was seeing was the map’s visibility data. This was pure conjecture that I didn't attempt to confirm until much later. I based this assumption on knowing that the visibility data was stored per-player, and that it was saved in the character file based on a glitch I had observed. 

(When I first started playing Valheim, I made a new world and invited a friend to join. He eventually decided to run a server on a spare computer and we decided to copy the world we had been playing on to the server. But I noticed that if any part of the map I explored on the multiplayer server also became explored on my singleplayer instance, as well as all the points-of-interest I had tagged etc. This meant that the game was using some aspect of the world (probably its seed, or name+seed pair) to store the map. This meant that the visibility data wasn’t stored in the map itself, but on the client side as part of the player’s character.)

I wrote the code above to skip over the 0x01’s and 0x00’s and tell me how many bytes were skipped. It turned out we were skipping a pretty similar amount several times based on how many maps the player had seen - and that number was 0x400000. 4.1 million bytes, or 4 megabytes. Approximately the entire file for a character that had only seen 1 map. Sometimes several more bytes got skipped, sometimes not. But I knew the data size was approximately that number, so I had something to look for in the header. And this time, I found it. 

![todo: add alt text](/screenshots/7.png?raw=true "todo: add title text")

A few bytes before the sea of 0x00’s and 0x01’s there was the number I sought: the number I needed to be able to ignore the map data for now. This is the size of the map data structure, all added up so you don’t need to step through it all byte-by-byte.  It was… bigger than I’d expected, with a size of 0x4010604 bytes it was 4,192 bytes larger. This offset landed me near the end of one of those islands I mentioned before - a group of length-prepended ASCII strings inside the sea of 0x00’s. 

![todo: add alt text](/screenshots/8.png?raw=true "todo: add title text")

Looking at the strings here I recognized that this was the points-of-interest from one of my maps, as they were mostly strings I had written in the game, spelling mistakes and all. The 17 bytes of higher-entropy binary data in between each one was easy to skip over for now. Later it would be trivial to see that this data is a position (three 32-bit floats, 12 bytes), a single byte defining the icon used, and 32 bits extra used for flags.

```c
typedef struct {
    PString name;
    float x;
    float y;
    float z;
    char type;
    int flags; // bit 24 = icon crossed out
} POI;
```

![todo: add alt text](/screenshots/9.png?raw=true "todo: add title text")

Following the first map, unsurprisingly, was another map. But this was an important clue to the size of the map header, because initially I only knew that the map was preceded by the size of the map but I did not know how many bytes before that belonged to the first map vs the file header. At the address indicated by the map size I’d found above, there was 70 bytes more data before the next field of 0x00’s. And, as with the first map there was a value just above 0x400000 right before the 0x00’s, this showed that the map structure here should be the same as the one above, and counting backwards allowed me to see how many bytes were before the map size in the map header. I now knew the overall structure of a map, even if I didn’t know what each byte meant yet:

```c
struct Map {
    char unknown[60];
    int map_size;
    char map[map_size];
}
```

# Finally something interesting
This at long last allowed me to identify, in any file i loaded, the starting point of the actual character. I still had a large area between the character’s first byte and the file-end structure I had identified, but it was mostly ascii strings so I was already in good shape understanding what I was looking at. 

![todo: add alt text](/screenshots/10.png?raw=true "todo: add title text")

I started out by looking at what was clearly the inventory list. Each ASCII-string is preceded by a byte that lists the length of the string. Perfect. I wanted to know the size of each inventory item so I could start identifying the interior contents, but some were different sizes. Luckily, it was pretty obvious what the cause was. There is another string inside each structure, the name of the player who made it. Ignoring that string, and the item’s name string, every inventory item is 33 bytes. 

In 010-template language (which is like C but struct definitions can be variably sized and are basically code-blocks):

```c
typedef struct {
    PString name;
    char unknown[33];
    PString creator_name;
} Item;
```

With a lot of trial and error, and comparing several items and several characters, I eventually figured out that the structure looks like this:

```c
typedef struct {
    PString name;
    int quantity;
    float durability;
    int column;
    int row;
    char equipped;
    int quality;
    int unknown; // these 4 bytes are still unclear
    uquad creator_id; // this I didn't figure out until much later
    PString creator_name;
} Item;
```

The quantity is obvious, because most of them were 1 except items I know I would have multiple of, like food. Figuring out the rest took some back-and-forth between different items, characters, and loading the game to see what it did if I changed a value. This is where the CRC-ignoring really helped as I hadn’t figured out the CRC yet (and I still haven’t bothered) and I wouldn’t have been able to test this had the game been enforcing that. 

It took me an embarrassingly long time to figure out the durability. I thought initially that it was more complicated than it is. It’s just a float. I think I had the hex editor in big-endian mode or something when I was initially looking at it and never realized it was as simple as it was. I ended up accidentally creating a mace with several billion-billions durability due to not realizing this. The fact that the equipped-state is just a char confused me at first as well, since all the other items are 4-byte values neatly aligned. 

Once I had that the rest of the character struct started coming together like an avalanche. I had all the borders of the puzzle filled in it was just a matter of finding the middle-peices. 

# fleshing out the character structure
```c
typedef struct {
    PString name;
    uquad player_id;
    char unknown_aa[10];    // I still haven’t figured this out
    float max_hp;           // doesn't seem to be used
    float current_hp;       // value appears to be recalculated at launch
    float stamina;
    char unknown_ab;
    float unknown_counter;  // starts at 99999 on a new char
    PString selected_gp;
    char unknown_b[8];      // another unknown cluster
    Inventory inventory;
    Compendium compendium;
    PString beard;
    PString hair;
    float unknown_float1;   // some floats, but unknown use
    float unknown_float2;
    float unknown_float3;
    float unknown_float4;
    float hair_color;
    float skin_color;
    int gender;
    int food_count;
    Food foods[food_count];
    int two;
    int skill_count;
    Skill skills[skill_count];
} Character;
```

The other interesting areas within the character struct are the compendium and the skills blocks. The compendium is easy to read as it is mostly ASCII strings, length-prepended in a byte. It was surprising how many sub-dictionaries there are inside the compendium, 8 overall. The game stores every item you’ve encountered, in several different sets. 

* Items seen
* Crafting benches seen (with highest level seen)
* Recipes unlocked
* Places seen (ie different types of dungeons)
* Another list?
* mob trophies collected
* Biomes seen
* Tutorials (I assume they mean times you’ve talked to the raven)

```c
typedef struct {
    PStringList items_seen;
    int len2;
    UpgradableEntry craftbenches_seen[len2];
    PStringList recipe_list;
    PStringList places;
    PStringList unknown_list;
    PStringList trophies;
    int biomes_seen;
    Biomes biomes[biomes_seen];
    int len8;
    PStringPair tutorials[len8];
} Compendium;
```
# Skills
The skills block was revealing. At this point I was looking at several different character files and I had noticed that these were 12-byte structures, prepended with a integer number of values. So naturally I had that set up like this:

```c
typedef struct {
    char unknown[12];
} Twelve;
```
And called it a day. But this was an issue, because there was some overlap with the file-end structure

![todo: add alt text](/screenshots/11.png?raw=true "todo: add title text")

After that I realized that the offset was inclusive of the initial 4-bytes, which made everything line up. I Still didn't know what these itesm were, and I left them as anonymous `Twelve` structs for the time being. Eventaully I started a new character and went over the file with a fine-toothed comb, saving copies and loading the game to change 1 thing at a time to verify my previous findings. Then I noticed I had gained a skill in running, and this section grew. The game only lists skills that your character has a level in, so with a new character this section has just 3 starter skills. 

```c
typedef enum SkillName {
    SKILL_SWORDS = 1,
    SKILL_KNIVES = 2,
    SKILL_CLUBS = 3,
    SKILL_POLEARMS = 4,
    SKILL_SPEARS = 5,
    SKILL_BLOCKING = 6,
    SKILL_AXES = 7,
    SKILL_BOWS = 8,
    // 9?
    // 10?
    SKILL_UNARMED = 11,
    SKILL_PICKAXES = 12,
    SKILL_WOODCUTTING = 13,
    // SKIP A FEW..?
    SKILL_JUMP = 100,
    SKILL_SNEAK = 101,
    SKILL_RUN = 102,
    SKILL_SWIM = 103
} SkillName;

typedef struct {
    SkillName id;
    float level;
    float progress;
} Skill;
```

It was easy to guess what was in each struct by now, it is clear that the format uses mostly ints, floats, and a few chars for small items, with lists always length-prepended and otherwise fixed-size items excepting strings. Checking through the skills my character has currently gave me a list of what enum values are used for the skill id, these are conveniently listed in the UI in the same order they appear in the file. 

# Filling in the pieces. 

Now that I could walk the entire file without skipping over anything, I comfortably knew I had all the “edge-pieces” of this puzzle filled in. But there was still a large amount left in the middle. Firstly, there is a header on each map that was almost entirely unknown to me. I was also hand-waving over skipping the map section, as I hadn’t figured out exactly how the game was reading the fog-of-war visibility data. Only the size of the entire map data section was stored, which included POI data. I knew how to decode the POI data at this point but I couldn' t include it in my template because I couldn’t describe where it started programatically, I could just see it visually. I knew about where it ended, but there appeared to be some padding that would sometimes be present and sometimes not. 

# The Maps
The regions of 00’s and 01’s I had seen were all 0x400000 bytes long so I hard coded that in. This seemed like it might not be the correct solution at first, but it has worked so far. Where does that leave you? 8 bytes short of what I visually knew was the POI list, so thats actually fine. I put two "unknown" ints into the template and called it good. 

I already knew how to read the POIs, but this wasn’t enough because there was also sometimes a padding byte at the end of the poi list, but it didn’t appear to actually be padding. First of all, sometimes it has a nonzero value, and second of all it doesn’t actually accomplish aligning the following byte to a nice alignment all the time. Often, it made the following byte 1-byte away from alignment.

I tried a number of different ways to decide if the padding byte should be there, including trying to incorrectly-align the following byte, assuming this was a bug in the serialization, as well as moving the padding around to different structures to let it only appear if there were POI’s (it sometimes appeared without them on some characters) only on the first two maps (for some reason this was a commonality in my test files).

Finally I just test on if we have reached the “end” of the map per the map size offset we are given, and insert that extra byte if we are not there yet. This is a great part of the strength of 010’s templating feature because you can write code in the structure definition and it will re-evaluate the structure size for every instance created. I don't view this as a perfect solution because it involves too much thinking on the part of the struct definition.

```c
typedef struct Map {
    char unknown_a[22];
    float x;
    float y;
    float z;
    char unknown_b[26];
    int map_size;

    local int map_end = FTell() + map_size; // FTell provides the address we are "currently"
                                            // on at this point in the struct processing.
                                            // this is evaulated for each instance of the struct
    VisibilityData fog_of_war;
    int unknown[2];
    mapPOIs pois;

    if (FTell() != map_end) {               // include 1 extra byte if we aren't at exactly
        char padding;                       // the address defined above
    }

} Map;
```

I create a local variable, ie a variable that isn’t actually in the struct on disk but 010 will compute each time it attempts to map this structure onto the bytes in the file, that stores the map end address. Then, when we get to the end of the map, if we aren’t at that address we add a byte. This is somewhat fragile in that there could in theory be multiple bytes here, but I haven’t encountered any maps like that yet. 

# current status

Finally, most important things are wrapped up. I still need to work out the crc check so that it is complete, but I can leave that for now. I also have several sections of unknowns, but hopefully I can chip away at these as I see more files. I have some idea of what some things are, but no confirmation so I've left it out of the template. I think the second int in the file is the version number, but no proof of that yet, etc.

I don't know what future changes the game dev(s) will make that affect the file layout, but most things are developed iteratively so even if the encasulation format is totally changed in a future version the underlaying data should change more slowly as the game grows.
