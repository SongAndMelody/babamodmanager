use crate::mods::functions_from_string as funcs;

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
fn merge_strings(mut left: String, mut right: String) -> String {
    // get the set of Lua Functions from each file
    let lhs = funcs(&left);
    let rhs = funcs(&right);
    // if the two are disjoint...
    if lhs.is_disjoint(&rhs) {
        // then we can just put one file after another and return it
        return concat_strings(left, right);
    }

    // otherwise, grab the intersections
    let intersections = lhs.intersection(&rhs);
    // and also whether any of them are baba functions
    let mut has_baba_native = false;
    for item in intersections {
        has_baba_native |= item.is_baba_native();
    }
    // if none of them are baba native...
    if !has_baba_native {
        // it's a matter of function renaming
        // grab the intersections again
        let intersections = lhs.intersection(&rhs);
        // then iterate over each one
        for item in intersections {
            // grab its name
            let name = item.name();
            // create new names for the left and right hand sides
            let mut left_func = name.clone();
            left_func.push_str(LEFT_HAND_SUFFIX);
            let mut right_func = name.clone();
            right_func.push_str(RIGHT_HAND_SUFFIX);
            // replace each instance of the function call in the files with the new name
            left = left.replace(&name, &left_func);
            right = right.replace(&name, &right_func);
        }
        // then we can concatenate them and return it
        return concat_strings(left, right);
    }

    todo!()
}

/// Concatenates two strings, putting the second at the end of the first.
fn concat_strings(mut left: String, right: String) -> String {
    left.push_str(&right);
    left
}


fn string_to_function_strings(file: String) -> Vec<String> {
    // Split the string at every use of `function`
    file.split("function")
        // split it again at every `end` without indentation,
        // then grab the first part (so before the end)
        .map(|x| x.split("\nend").next())
        // we should have at least something before the end
        // so this is just type casting from
        // Option<&str> -> &str
        .map(Option::unwrap_or_default)
        // &str -> String
        .map(ToOwned::to_owned)
        // puts the `function` back on the front of the string
        .map(|str| concat_strings("function".to_owned(), str))
        // puts the `end` on the back of the string
        .map(|str| concat_strings(str, "\nend".to_owned()))
        // collect it into a list
        .collect()
}