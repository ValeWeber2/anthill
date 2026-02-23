#![allow(dead_code)]

use ratatui::style::{Color, Style};

/// Represents the basic building block of the world.
///
/// The `World` consists of `WORLD_WIDTH` x `WORLD_WIDTH` (default: 100x25) Tiles.
/// Tiles stand for the static environment of the world, not entities.
#[derive(Clone, Copy, Debug)]
pub struct Tile {
    /// Type of the tile
    pub tile_type: TileType,
    /// Whether the tile is currently visible by the player.
    pub visible: bool,
    /// Whether the tile has ever been seen by the player.
    /// Non-visible, previously explored areas appear gray.
    pub explored: bool,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self { tile_type, visible: false, explored: false }
    }

    /// Reveal the tile to the player.
    pub fn make_visible(&mut self) {
        self.visible = true;
    }

    /// Conceal the tile for the player.
    pub fn make_invisible(&mut self) {
        self.visible = false;
    }

    /// Mark this tile as previously explored.
    pub fn make_explored(&mut self) {
        self.explored = true;
    }
}

impl Default for Tile {
    fn default() -> Self {
        Self { tile_type: TileType::Void, visible: false, explored: false }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TileType {
    /// Out-of-bounds space outside the playable area.
    /// Space between rooms that cannot be seen or walked through. In-fiction this represents solid rock.
    Void,

    /// Basic walkable floor
    Floor,

    /// Walls that encase every room.
    Wall,

    /// Hallways between rooms. (Not surrounded by walls)
    Hallway,

    /// Door that leads from a room's [TileType::Wall] to a [TileType::Hallway]
    Door(DoorType),

    /// Stairs that lead further down into the dungeon
    StairsDown,

    /// Stairs that lead back up the dungeon floors
    StairsUp,
}

impl std::fmt::Display for TileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TileType::Void => write!(f, "Nothing"),
            TileType::Floor => write!(f, "Floor"),
            TileType::Wall => write!(f, "Wall"),
            TileType::Hallway => write!(f, "Hallway"),
            TileType::Door(DoorType::Archway) => write!(f, "Archway"),
            TileType::Door(DoorType::Closed) => write!(f, "Closed Door"),
            TileType::Door(DoorType::Open) => write!(f, "Open Door"),
            TileType::StairsDown => write!(f, "Stairs leading further down..."),
            TileType::StairsUp => write!(f, "Stairs leading back up."),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DoorType {
    /// The door is open and cannot be closed again.
    Open,

    /// The door is closed and must be interacted with to open.
    Closed,

    /// No door is present. Basically just a hole in the wall.
    Archway,
}

/// A trait for giving something a visual representation in the TUI style.
pub trait Drawable {
    /// Returns the unicode `char` to be used in the graphical representation.
    fn glyph(&self) -> char;

    /// Returns the [ratatui] [Style] to be used in the graphical representation.
    fn style(&self) -> Style;
}

/// A trait for defining whether something can be walked through by the player and NPCs or not.
pub trait Collision {
    /// Returns a boolean denoting whether something can be walked through or not.
    fn is_walkable(&self) -> bool;
}

/// A trait for defining whether an object is opaque or see-through.
///
/// This is used in the field-of-view system, determining player vision.
pub trait Opacity {
    /// Returns a boolean denoting whether something is opaque (`true`) or see-through (`false`).
    fn is_opaque(&self) -> bool;
}

/// A trait for defining if an object is interactable and what game interaction it creates.
pub trait Interactable {
    /// Returns a boolean denoting whether something is interactable (`true`) or has no defined interactions (`false`).
    fn is_interactable(&self) -> bool;
}

impl Collision for TileType {
    fn is_walkable(&self) -> bool {
        match self {
            TileType::Void => false,
            TileType::Floor => true,
            TileType::Wall => false,
            TileType::Hallway => true,
            TileType::Door(DoorType::Open) => true,
            TileType::Door(DoorType::Closed) => false,
            TileType::Door(DoorType::Archway) => true,
            TileType::StairsDown => true,
            TileType::StairsUp => true,
        }
    }
}

impl Drawable for TileType {
    /// # Note
    /// Walls are rendered using directional wall characters, which are calculated in post-processing during render.
    fn glyph(&self) -> char {
        match self {
            TileType::Void => ' ',
            TileType::Floor => '·',
            TileType::Wall => '#', // Will not be displayed Is replaced with a directional wall character instead.
            TileType::Hallway => '░',
            TileType::Door(DoorType::Archway) => '·',
            TileType::Door(DoorType::Open) => '_',
            TileType::Door(DoorType::Closed) => '+',
            TileType::StairsDown => '>',
            TileType::StairsUp => '<',
        }
    }
    fn style(&self) -> Style {
        match self {
            TileType::Void => Style::default(),
            TileType::Floor => Style::default().fg(Color::Gray),
            TileType::Wall => Style::default().fg(Color::White),
            TileType::Hallway => Style::default().fg(Color::DarkGray),
            TileType::Door(DoorType::Archway) => Style::default().fg(Color::Gray),
            TileType::Door(_) => Style::default().fg(Color::Yellow),
            TileType::StairsDown => Style::default().fg(Color::White),
            TileType::StairsUp => Style::default().fg(Color::White),
        }
    }
}

impl Opacity for TileType {
    fn is_opaque(&self) -> bool {
        match self {
            TileType::Void => true,
            TileType::Floor => false,
            TileType::Wall => true,
            TileType::Hallway => false,
            TileType::Door(DoorType::Open) => false,
            TileType::Door(DoorType::Closed) => true,
            TileType::Door(DoorType::Archway) => false,
            TileType::StairsDown => false,
            TileType::StairsUp => false,
        }
    }
}

impl Interactable for TileType {
    fn is_interactable(&self) -> bool {
        match self {
            TileType::Void => false,
            TileType::Floor => false,
            TileType::Wall => false,
            TileType::Hallway => false,
            TileType::Door(DoorType::Open) => false,
            TileType::Door(DoorType::Closed) => true,
            TileType::Door(DoorType::Archway) => false,
            TileType::StairsDown => true,
            TileType::StairsUp => true,
        }
    }
}
