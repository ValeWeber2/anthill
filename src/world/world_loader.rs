use std::fs::File;
use std::io::BufReader;

use ron::de::from_reader;

use crate::util::errors_results::{GameError, IoError};
use crate::world::world_data::WorldData;

pub fn load_world_from_ron(path: &str) -> Result<WorldData, GameError> {
    let file = File::open(path).map_err(IoError::MapReadFailed)?;
    let reader = BufReader::new(file);
    let data: WorldData = from_reader(reader).map_err(IoError::MapParseFailed)?;
    Ok(data)
}
