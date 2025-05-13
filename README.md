# Baba Mod Manager

A Mod manager for Baba is You. Currently a work in progress.

## Usage - Developers

The mod manager is intended to be backwards-compatible with any mod, even those that are just singletons and the like. However, you can add some extra flair to your mod for the cost of a little bit of tweaking.

Set up your mod like so in the file system:

```txt
.../Lua
    |- YourMod
    |  |- [mod files]
    |  |- Config.json // more on this later!
    |- YourMod_init.lua
```

`YourMod_init.lua` should call into the mod files inside the `YourMod` directory. You can then add data to `Config.json` to spruce up the mod in the manager. You can use this as a template:

```json
{
    "modid": "dummytest", // String
    "authors": ["Author A", "Author B"], // [String]
    "description": "A very cool description for this mod", // String
    "icon_url": "[Replace this with a url to an icon, either locally or on the net]", // String (optional)
    "banner_url": "[See above, but for a banner]", // String (optional)
    "global": false, // Whether the mod is to be installed globally or in a levelpack (boolean)
    "tags": ["Technical", "Work In Progress"], // [String], any past 4 are ignored
    "links": ["[You can put links here to forward people to the right places]", "[You can have multiple!]"], // [String], can be length 0 if N/A
    "files": ["[This is a list of files that are considered part of the mod, and are moved with it when requested]"], // [String], list relative paths to files here
    "init": "[The file used outside of the folder, if needed.]", // String,
    "sprites": ["[This is a set of sprites the mod uses]"] // [String]
}
```

## Attribution

### Fonts

The Baba is You font was taken from the Robot is Chill repository: <https://github.com/ROBOT-IS-CHILL/robot-is-chill/blob/main/data/fonts/ui.ttf>

LibMono, LibSans, and LibSerif are all from the Liberation Font Series: <https://github.com/liberationfonts>

Dyslecic and DyslexicMono are from OpenDyslexic: <https://opendyslexic.org/>

### Palettes

Abstract, Autumn, Crystal, Default, Factory, garden, High Contrast, Marshmallow, Monochromatic Grey, Monochromatic Red, Mountain, Ocean, Ruins, Space, Swamp, Vibrant and Volcano are all taken directly from Baba is You.

Haven is Babaisyoo's "Resting Haven" palette.

Graves, Pollution, Shogun, and Thunders are taken from [D3vland](https://d3vq.itch.io/baba-d3vland) by D3v0553.

Pico-8 and Win95 are taken from their respective operating systems' palettes.

Godot is based off of Godot's text editor's colors, collated by Dogeiscut.

Sheaflet's colors are taken from [Rosy42](https://lospec.com/palette-list/rosy-42) by PineappleOnPizza.

### Copyright Attribution

Steam's Logo: © 2025 Valve Corporation. All rights reserved. Valve, as well as the Steam logo, are trademarks and/or registered trademarks of Valve Corporation. [Copyright Notice](https://store.steampowered.com/legal/)
