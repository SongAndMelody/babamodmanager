use diff_match_patch_rs::{DiffMatchPatch, PatchInput};

use crate::{
    error::{babaerror::BabaError, moddingerror::ModdingError},
    files::luafile::LuaFile,
    mods::{babamod::BabaMod, concat_strings, config::Config, luafunction::LuaFunction},
};

/// Defines the prefix of a lua function,
/// if duplicates are found, and it is
/// on the *left* hand side of the arguments
const LEFT_HAND_SUFFIX: &str = "_left";
/// Defines the prefix of a lua function,
/// if duplicates are found, and it is
/// on the *right* hand side of the arguments
const RIGHT_HAND_SUFFIX: &str = "_right";

/// The mode used by [`DiffMatchPatch`].
/// This can be one of two types:
/// - [`diff_match_patch_rs::Compat`] - return types deal with [`char`]s and slices thereof.
/// - [`diff_match_patch_rs::Efficient`] - return types deal exclusively in `&[u8]` slices.
type DiffMode = diff_match_patch_rs::Compat;

/// Attempts to merge two [`LuaFile`]s.
/// # Semantics
/// - The order of parameters matter - the two files are merged into one, with the
/// left parameter coming first, and the second parameter coming after. In other words,
/// the left parameter has priority.
/// - Functions are only merged if they both override a function from Baba is You.
/// Otherwise, they are renamed with additional suffixes - see [`LEFT_HAND_SUFFIX`] and [`RIGHT_HAND_SUFFIX`]
/// for specifics on those values.
/// - In the case where functions are merged, the file is ordered with the left file's data first,
/// then merged data, then the right file's data.
/// # Errors
/// This function will only error if merging is not possible in some way, shape, or form.
/// Specifics:
/// - Will return [`ModdingError::RenameError`] if, while attempting to merge an Injected and Overridden mod
/// (see below), the dictionary of renamed variables was not properly set in the mod with the injected function.
/// - Will return [`BabaError::DmpError`] as per the specifications of [`merge_override_functions`] or [`merge_injected_functions`],
/// depending on whether both mods use the Override or Injection method.
/// ## Override vs Injection
/// When it comes to baba modding, there are two ways to replace a function native to baba.
/// While they are unnamed, the first way is known to this program as the "override" method.
/// This is done via copying the function's code from the game and tweaking the function
/// to include your own code.
/// It looks something like this:
/// ```lua
/// function init(args...)
///     -- game code
///     do_something()
///     -- more game code
/// end
/// ```
///
/// The second method is known as the "injection" method. This is done by copying the function
/// via way of creating a variable that holds the copy in the code, and then calling the old function
/// inside a new function that overwrites the original. It looks something like this:
/// ```lua
/// local oldinit = init
/// function init(args...)
///     do_something()
///     init(args...)
/// end
/// ```
/// This is generally used as a way to make mod merging easier. Nontheless, this function
/// supports both types of function replacement, and can also work via mix-and-matching with
/// a custom merging solution (via way of including both the modified code, and the injection function).
/// ## UNusual Function Declarations
/// Any of the following are not supported, but can be easily refactored into either one of the other two.
/// ```lua
/// -- "Direct Injection"
/// menufuncs.[menu].enter = function(args...)
///     -- This can be easily refactored into a proper form
///     -- via moving the `function` to the start
///     -- `function menufuncs.[menu].enter(args...)`
///     -- The program does this automatically
/// end
/// ```
#[must_use]
pub fn merge_files(
    left_file: LuaFile,
    right_file: LuaFile,
    baba_funcs: &[LuaFunction],
) -> Result<LuaFile, BabaError> {
    let mut left = left_file.code();
    let mut right = right_file.code();
    let mut merged = String::new();
    // get the set of Lua Functions from each file
    let lhs = left_file.definitions();
    let rhs = right_file.definitions();

    // grab the intersections
    let intersections = lhs.intersection(&rhs);
    // iterate over the intersections
    // if there are none, the loop does nothing
    // and we just return the two files concatenated (see below)
    for func in intersections {
        // if it is not native to baba...
        if !func.is_baba_native() {
            // we can just rename the functions
            // grab its name
            let name = func.name();
            // create new names for the left and right hand sides
            let mut left_func = name.clone();
            left_func.push_str(LEFT_HAND_SUFFIX);
            let mut right_func = name.clone();
            right_func.push_str(RIGHT_HAND_SUFFIX);
            // replace each instance of the function call in the files with the new name
            left = left.replace(&name, &left_func);
            right = right.replace(&name, &right_func);
            continue;
        }
        // it IS native to baba
        // grab the functions from each file
        let lhs = LuaFunction::from_definition_and_code(func, &left);
        let rhs = LuaFunction::from_definition_and_code(func, &right);
        let (Some(left_func), Some(right_func)) = (lhs, rhs) else {
            continue;
        };
        // remove the code from the files
        // We'll be appending it later to the merged section
        left = left.replace(left_func.code(), "");
        right = right.replace(right_func.code(), "");
        // check: we want to ensure that no functions are merged if only one function
        // uses the injection method
        let new_func = match (
            left_file.function_uses_injection(&left_func.definition()),
            right_file.function_uses_injection(&right_func.definition()),
        ) {
            // only one function uses the injection method
            (true, false) | (false, true) => {
                // this binding ensures that `injected` is *always* the injected method
                let (injected, not_injected, rename) =
                    if left_file.function_uses_injection(&left_func.definition()) {
                        (left_func, right_func, left_file.injection_data(func))
                    } else {
                        (right_func, left_func, right_file.injection_data(func))
                    };
                // The non-injected version needs to go first
                merged.push_str(not_injected.code());
                // then we add the variable definition that allows the
                // injected version to work
                let Some(rename) = rename else {
                    return Err(ModdingError::RenameError)?;
                };
                let name = func.name();
                let line = format!("local {} = {}", rename, name);
                merged.push('\n');
                merged.push_str(&line);
                // then we add the injection version of the function
                merged.push('\n');
                merged.push_str(injected.code());
                continue;
            }
            // neither function uses the injection method
            (false, false) => merge_override_functions(left_func, right_func, baba_funcs)?,
            // both functions use the injection method
            (true, true) => merge_injected_functions(left_func, right_func)?,
        };
        merged.push('\n');
        merged.push_str(new_func.code());
        merged.push('\n');
    }
    // Now that all the issues have been ironed out,
    // we can concatenate the two files together
    // with no issues! hopefully
    let mut result = concat_strings(left, concat_strings(merged, right));

    // Some final touch ups:
    // remove any instances of double line breaks
    while result.contains("\n\n") {
        result = result.replace("\n\n", "\n");
    }
    Ok(result.into())
}

/// Merges two Lua Functions, assuming both are override functions.
/// # Prereqs
/// - Both functions should be checked beforehand to ensure they do not use the injection method.
/// - Additionally, both functions should have the same [`crate::mods::LuaFuncDef`].
/// - The third parameter should have at least one [`LuaFunction`] that has the same definition
/// as the other two
///
/// # Errors
/// This errors under a couple circumstances:
/// - The function could not be properly merged
/// - Either set of code removes code form the original function its based on
/// - After merging, for whatever reason, it was not considered a valid function
/// - The third parameter did not contain an original function to match the other two
/// - Either function removes tokens from the original code (considered too code-changing to merge)
pub fn merge_override_functions(
    left: LuaFunction,
    right: LuaFunction,
    baba_funcs: &[LuaFunction],
) -> Result<LuaFunction, BabaError> {
    use diff_match_patch_rs::Ops;

    let original = baba_funcs
        .iter()
        .find(|&func| func.definition() == left.definition())
        .ok_or(ModdingError::NotABabaFunction)?
        .clone();

    let dmp = DiffMatchPatch::new();
    // grab the diffs between the files and the code of the original function
    let diffs_left = dmp.diff_main::<DiffMode>(original.code(), left.code())?;
    let diffs_right = dmp.diff_main::<DiffMode>(original.code(), right.code())?;
    // check if any tokens are removed
    for diff in diffs_left.iter().chain(diffs_right.iter()) {
        match diff.op() {
            // In the case of removal, we want to immediately quit
            // since mods that remove code probably don't want to be merged
            Ops::Delete => return Err(BabaError::ModdingError(ModdingError::CodeRemoval)),
            Ops::Equal | Ops::Insert => continue,
        }
    }
    merge_functions_via_dmp(left, right)
}

/// Merges two Lua Functions, assuming both are injected functions.
/// # Prereqs
/// - Both functions should be checked beforehand to ensure they do not use the override method.
/// - Additionally, both functions should have the same [`crate::mods::LuaFuncDef`].
/// - The third parameter should have at least one [`LuaFunction`] that has the same definition
/// as the other two
///
/// # Errors
/// This errors under a couple circumstances:
/// - The function could not be properly merged
/// - Either set of code removes code form the original function its based on
/// - After merging, for whatever reason, it was not considered a valid function
/// - The third parameter did not contain an original function to match the other two
pub fn merge_injected_functions(
    left: LuaFunction,
    right: LuaFunction,
) -> Result<LuaFunction, BabaError> {
    // in this case, the injected functions are small enough to where
    // we don't need to check for deletion tokens
    // (they are removed anyways in the following function call)
    merge_functions_via_dmp(left, right)
}

/// Merges two lua functions, just by code
/// Do not use this, use [`merge_override_functions`] or [`merge_injected_functions`]
fn merge_functions_via_dmp(
    left: LuaFunction,
    right: LuaFunction,
) -> Result<LuaFunction, BabaError> {
    use diff_match_patch_rs::Ops;

    let dmp = DiffMatchPatch::new();
    // now we can start merging!
    // we grab the differences between the left and right function
    let diffs = dmp.diff_main::<DiffMode>(left.code(), right.code())?;
    // remove the removal tokens since none should exist (and would only exist since the two functions are different)
    let diffs: Vec<_> = diffs
        .into_iter()
        .filter(|diff| diff.op() != Ops::Delete)
        .collect();
    // create patches from the diffs
    let patches = dmp.patch_make(PatchInput::new_text_diffs(left.code(), &diffs))?;
    // apply them
    let (result, flags) = dmp.patch_apply(&patches, left.code())?;
    for flag in flags {
        if !flag {
            return Err(BabaError::ModdingError(ModdingError::IncompletePatching));
        }
    }
    Ok(result.parse()?)
}

fn config_from_two_mods(left: &BabaMod, right: &BabaMod) -> Config {
    let id = concat_strings(left.mod_id(), right.mod_id()).replace('\n', "");
    let left_name = left.name();
    let right_name = right.name();
    let left_auth = format!("{:?}", left.authors())
        .replace('[', "")
        .replace(']', "");
    let right_auth = format!("{:?}", right.authors())
        .replace('[', "")
        .replace(']', "");
    let left_desc = left.description();
    let right_desc = right.description();

    let config = serde_json::json! ({
        "modid": format!("{id}"),
        "authors": [format!("Authors of {left_name}: {left_auth}"), format!("Authors of {right_name}: {right_auth}")],
        "description": format!("A merger between {left_name} and {right_name}, automatically generated by BMM.\n{left_name}:{left_desc}\n{right_name}:{right_desc}"),
        "icon_url": "",
        "banner_url": "",
        "global": false,
        "tags": ["Auto-generated", "Merged"],
        "links": ["[Intentionally left without links]"],
        "files": ["[Intentionally left without files]"],
        "init": format!(".\\{id}_init.lua"),
        "sprites": ["[Intentionally left without names]"]
    });

    // this function *should not fail* so we should abort early if needed
    let result = serde_json::from_value(config).expect("Given `config` binding in this function should always be able to be parsed into a `Config` structure");
    result
}

/// Merges two mods, creating a new one in the same folder.
pub fn merge_mods(
    left: &BabaMod,
    right: &BabaMod,
    _funcs: Vec<LuaFunction>,
) -> Result<BabaMod, BabaError> {
    let _config = config_from_two_mods(left, right);

    todo!()
}
