use std::str::FromStr;

use crate::error::levelpackerror::LevelpackError;

pub mod levelpack;
pub mod levelpackfile;

/// The name of the file that holds the world data
pub const WORLD_DATA_FILE_NAME: &str = "world_data.txt";

/// Attempts to get and parse a field from a line of text.
///
/// For example: giving `fetch_field<usize>("name", "name=abc")`
/// should return `Ok("abc")`
///
/// # Errors
/// This function may error if:
/// - When parsing the field, it did not match the `"lhs=rhs"` format
/// - The left hand side did not match the `field` parameter
/// - The right hand side could not be properly parsed into the desired type
pub fn fetch_field<T>(field: &str, data: &str) -> Result<T, LevelpackError>
where
    T: FromStr,
{
    let split = data
        .split_once('=')
        .ok_or(LevelpackError::FieldParsingError(true))?;
    if split.0 != field {
        Err(LevelpackError::FieldParsingError(false))
    } else {
        split
            .1
            .parse()
            .map_err(|_| LevelpackError::StringParsingError("Malformed world_data.txt".to_owned()))
    }
}
