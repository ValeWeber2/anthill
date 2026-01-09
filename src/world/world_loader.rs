use std::fs::File;
use std::io::BufReader;

use ron::de::from_reader;

use crate::world::world_data::WorldData;

pub fn load_world_from_ron(path: &str) -> Result<WorldData, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data: WorldData = from_reader(reader)?;
    Ok(data)
}
