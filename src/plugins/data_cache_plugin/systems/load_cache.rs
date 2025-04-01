use bevy::utils::hashbrown::HashMap;
use serde::de::DeserializeOwned;
use std::fs;

pub fn load_cache<T: DeserializeOwned + 'static>(
    path: &str,
    cache: &mut HashMap<i32, T>,
) -> Result<(), String> {
    let file_content =
        fs::read_to_string(path).map_err(|_| format!("Failed to read data from {}", path))?;

    let definitions: HashMap<i32, T> = serde_json::from_str(&file_content)
        .map_err(|_| format!("Invalid JSON format in file: {}", path))?;

    cache.extend(definitions);
    Ok(())
}
