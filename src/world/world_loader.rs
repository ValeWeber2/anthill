#![allow(dead_code)]

use std::fs::File;
use std::io::{BufReader, BufWriter};

use ron::de::from_reader;
use ron::ser::{PrettyConfig, to_writer_pretty};

use crate::util::errors_results::{GameError, IoError};
use crate::world::world_data::WorldData;

pub fn load_world_from_ron(path: &str) -> Result<WorldData, GameError> {
    let file = File::open(path).map_err(IoError::FileReading)?;
    let reader = BufReader::new(file);
    let data: WorldData = from_reader(reader).map_err(IoError::MapParsing)?;
    Ok(data)
}

pub fn save_world_to_ron(world_data: &WorldData, path: &str) -> Result<(), GameError> {
    let file = File::create(path).map_err(IoError::FileCreation)?;
    let writer = BufWriter::new(file);
    to_writer_pretty(writer, world_data, PrettyConfig::default()).map_err(IoError::MapWriting)?;
    Ok(())
}
