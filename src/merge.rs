use std::str::FromStr;

use crate::{error::BabaError, mods::{concat_strings, functions_from_string as funcs, LuaFunction}};

use diff::{lines as diff_lines, Result as Diff};

/// Defines the prefix of a lua function,
/// if duplicates are found, and it is
/// on the *left* hand side of the arguments
const LEFT_HAND_SUFFIX: &str = "_left";
/// Defines the prefix of a lua function,
/// if duplicates are found, and it is
/// on the *right* hand side of the arguments
const RIGHT_HAND_SUFFIX: &str = "_right";

/// Attempts to merge two strings, as if they were Lua files.
/// # Technicalities
/// - The order of parameters matter - the two Strings are merged into one, with the
/// left parameter coming first, and the second parameter coming after.
/// - Functions are only merged if they both override a function from Baba is You.
/// Otherwise, they are renamed with additional suffixes - see [`LEFT_HAND_SUFFIX`] and [`RIGHT_HAND_SUFFIX`]
/// for specifics on those values.
/// - In the case where functions are merged, the file is ordered with the left file's data first,
/// then merged data, then the right file's data.
/// # Errors
/// This function will only error if merging is not possible in some way, shape, or form. 
fn merge_strings(mut left: String, mut right: String) -> Result<String, BabaError> {
    // get the set of Lua Functions from each file
    let lhs = funcs(&left);
    let rhs = funcs(&right);

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
            let lhs= LuaFunction::from_definition_and_code(func, &left);
            let rhs = LuaFunction::from_definition_and_code(func, &right);
            // if the function doesn't actually exist in both files, we can skip it
            // NOTE: Might be a good idea to check why?
            // I think in most instances this is just a guarded Option<LuaFunction> -> LuaFunction cast
            let (Some(left_func), Some(right_func)) = (lhs, rhs) else {
                continue;
            };
            // remove the code from the files
            // We'll be appending it later to the left file
            left = left.replace(left_func.code(), "");
            right = right.replace(right_func.code(), "");
            // merge the functions
            let new_func = merge_functions(left_func, right_func)?;
            // merge it onto the left file
            left.push_str(&new_func.code());
        }
    }
    // Now that all the issues have been ironed out,
    // we can concatenate the two files together
    // with no issues! hopefully
    Ok(concat_strings(left, right))
}


/// Merges two Lua Functions
/// # Errors
/// This errors under one of two circumstances:
/// - The function could not be properly merged
/// - After merging, for whatever reason, it was not considered a valid function
fn merge_functions(left: LuaFunction, right: LuaFunction) -> Result<LuaFunction, BabaError> {
    let mut result = String::new();
    for line in diff_lines(left.code(), right.code()) {
        match line {
            // TODO: figure out how this works
            Diff::Left(_) => todo!(),
            Diff::Both(_, _) => todo!(),
            Diff::Right(_) => todo!(),
        }
    }
    Ok(result.parse()?)
}
