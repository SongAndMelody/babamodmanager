#![cfg(test)]

use crate::{application::{icon, load_fonts, load_themes, pixel_index}, files::babafiles::BabaFiles, levelpack::fetch_field as ff, mods::config::Config};

/// Tests whether or not `fetch_field` returns an `Ok` variant
#[test]
fn fetch_field_1() {
    let x: Result<String, _> = ff("name", "name=abc");
    assert!(x.is_ok());
}

/// Tests whether or not `fetch_field` returns the proper value
#[test]
fn fetch_field_2() {
    let x: Result<String, _> = ff("name", "name=abc");
    let x = x.unwrap();
    assert_eq!(x, "abc");
}

#[test]
fn find_baba_files() {
    let x = BabaFiles::from_steam();
    assert!(x.is_ok())
}

#[test]
fn levelpacks_are_created() {
    let files = BabaFiles::from_steam().unwrap();
    let packs = files.levelpacks(false);
    assert!(packs.is_ok(), "packs is not ok: {:?}", packs)
}

#[test]
fn levelpacks_exist() {
    let files = BabaFiles::from_steam().unwrap();
    let packs = files.levelpacks(false).unwrap();
    assert!(packs.len() != 0, "{:?}", packs);
}

#[test]
fn can_fetch_mods() {
    let files = BabaFiles::from_steam().unwrap();
    let packs = files.levelpacks(false).unwrap();
    let mods = packs[0].mods();
    assert!(mods.is_ok());
}

#[test]
fn mods_exist() {
    let files = BabaFiles::from_steam().unwrap();
    let packs = files.levelpacks(false).unwrap();
    let mods = packs
        .into_iter()
        .map(|pack| pack.mods())
        .flatten()
        .flatten()
        .collect::<Vec<_>>();
    assert!(mods.len() != 0, "{:?}", mods);
}

#[test]
fn can_parse_config() {
    use serde_json::json;
    let json = json!({
      "modid": "dummytest",
      "authors": ["Author A", "Author B"],
      "description": "A very cool description for this mod",
      "icon_url": "[Replace this with a url to an icon, either locally or on the net]",
      "banner_url": "[See above, but for a banner]",
      "global": false,
      "tags": ["Technical", "Work In Progress"],
      "links": ["[You can put links here to forward people to the right places]", "[You can have multiple!]"],
      "files": ["[This is a list of files that are considered part of the mod, and are moved with it when requested]"],
      "init": "[The file used outside of the folder, if needed.]",
      "sprites": ["[This is a set of sprites the mod uses]"]
    });
    let value = Config::from_json(json);
    assert!(value.is_ok(), "{:?}", value.err().unwrap());
}

#[test]
fn load_palette_data() {
    let value = load_themes();
    assert!(value.is_ok(), "{:?}", value.err().unwrap())
}

#[test]
fn correct_amount_of_palettes() {
    let value = load_themes().unwrap();
    assert!(value.len() == 20, "{:?}", value);
}

#[test]
fn load_font_data() {
    let value = load_fonts();
    assert!(value.is_ok(), "{:?}", value.err().unwrap())
}

#[test]
fn correct_amount_of_fonts() {
    let value = load_fonts().unwrap();
    assert!(value.len() == 6, "{:?}", value);
}

#[test]
fn pixel_index_generic() {
    assert_eq!(pixel_index(2, 1), 9)
}

#[test]
fn pixel_index_min() {
    assert_eq!(pixel_index(0, 0), 0)
}

#[test]
fn pixel_index_max() {
    assert_eq!(pixel_index(6, 4), 34)
}

#[test]
fn icon_load() {
    let icon = icon();
    assert!(icon.is_ok(), "{:?}", icon)
}