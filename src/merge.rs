use std::str::FromStr;

use crate::{
    error::BabaError,
    mods::{concat_strings, functions_from_string as funcs, LuaFile, LuaFunction, ModdingError},
};

use diff_match_patch_rs::DiffMatchPatch;

/// Defines the prefix of a lua function,
/// if duplicates are found, and it is
/// on the *left* hand side of the arguments
const LEFT_HAND_SUFFIX: &str = "_left";
/// Defines the prefix of a lua function,
/// if duplicates are found, and it is
/// on the *right* hand side of the arguments
const RIGHT_HAND_SUFFIX: &str = "_right";

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
fn merge_strings(mut left_file: LuaFile, mut right_file: LuaFile) -> Result<LuaFile, BabaError> {
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
        } else {
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
            let injection_check = left_file.function_uses_injection(&left_func.definition())
                ^ right_file.function_uses_injection(&right_func.definition());
            if injection_check {
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
            } else {
                // Otherwise, we can merge the functions like normal.
                let new_func = merge_functions(left_func, right_func)?;
                merged.push_str(new_func.code());
            }
        }
    }
    // Now that all the issues have been ironed out,
    // we can concatenate the two files together
    // with no issues! hopefully
    let result = concat_strings(left, concat_strings(merged, right));

    // Some final touch ups:
    // remove any instances of double line breaks
    let result = result.replace("\n\n", "\n");
    Ok(result.into())
}

/// Merges two Lua Functions
/// # Errors
/// This errors under one of two circumstances:
/// - The function could not be properly merged
/// - After merging, for whatever reason, it was not considered a valid function
pub fn merge_functions(left: LuaFunction, right: LuaFunction) -> Result<LuaFunction, BabaError> {
    let mut result = String::new();
    let dmp = DiffMatchPatch::new();
    todo!("figure out the merging semantics");
    Ok(result.parse()?)
}
