# Documentation for the data folder

The data folder contains a couple of files the program uses as either templates or data to retreive.
Listed below is an explanation of all of them.

## !doc.md

This file, which contains the documentation for all the files in the data folder.

## babafuncs.txt

Contains a list of all 338 overridable Baba is You functions. These are used often, since the primary way to modify Baba is You is to override functions from the basegame.

## Config.json

This exists for two reasons:

- A template to work off of for those wanting to make mods compatible with the mod manager (see the readme)
- A template used when merging two functions - the file is copied and modified to allow compatibility with the mod manager for the newly merged mods.

## init.lua

This file is similarly used when merging two mods, calling into a function in the directory for easy use.
