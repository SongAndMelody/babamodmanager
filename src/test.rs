
/// Tests whether or not `fetch_field` returns an `Ok` variant
#[test]
fn fetch_field_1() {
    use crate::levelpack::fetch_field as ff;
    let x: Result<String, _> = ff("name", "name=abc");
    assert!(x.is_ok());
}

/// Tests whether or not `fetch_field` returns the proper value
#[test]
fn fetch_field_2() {
    use crate::levelpack::fetch_field as ff;
    let x: Result<String, _> = ff("name", "name=abc");
    let x = x.unwrap();
    assert_eq!(x, "abc");
}

#[test]
fn find_baba_files() {
    use crate::baba::BabaFiles;
    let x = BabaFiles::from_steam();
    assert!(x.is_ok())
}

#[test]
fn levelpacks_are_created() {
    use crate::baba::BabaFiles;
    let files = BabaFiles::from_steam().unwrap();
    let packs = files.levelpacks(false);
    assert!(packs.is_ok(), "packs is not ok: {:?}", packs)
}

#[test]
fn levelpacks_exist() {
    use crate::baba::BabaFiles;
    let files = BabaFiles::from_steam().unwrap();
    let packs = files.levelpacks(false).unwrap();
    assert!(packs.len() != 0, "{:?}", packs);
}

#[test]
fn can_fetch_mods() {
    use crate::baba::BabaFiles;
    let files = BabaFiles::from_steam().unwrap();
    let packs = files.levelpacks(false).unwrap();
    let mods = packs[0].mods();
    assert!(mods.is_ok());
}

#[test]
fn mods_exist() {
    use crate::baba::BabaFiles;
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
    use crate::mods::Config;
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
      "sprites": ["[This is a set of sprites the mod uses]"]
    });
    let value = Config::from_json(json);
    assert!(value.is_ok(), "{:?}", value.err());
}
