#![allow(dead_code)]

use ratatui::style::{Color, Style};

use crate::world::worldspace::{Collision, Drawable};

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub tile_type: TileType,
    pub visible: bool,
    pub explored: bool,
}

impl Tile {
    pub fn new(tile_type: TileType) -> Self {
        Self { tile_type, visible: false, explored: false }
    }

    pub fn make_visible(&mut self) {
        self.visible = true;
    }

    pub fn make_invisible(&mut self) {
        self.visible = false;
    }

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
    Void,
    Floor,
    Wall,
    Hallway,
    Door(DoorType),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DoorType {
    Open,
    Closed,
    Archway,
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
        }
    }
}

impl Drawable for TileType {
    fn glyph(&self) -> char {
        match self {
            TileType::Void => ' ',
            TileType::Floor => '·',
            TileType::Wall => '#', // Will not be displayed Is replaced with a directional wall character instead.
            TileType::Hallway => '▒',
            TileType::Door(DoorType::Open) => '_',
            TileType::Door(DoorType::Closed) => '+',
            TileType::Door(DoorType::Archway) => '·',
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
        }
    }
}

impl TileType {
    pub fn is_opaque(&self) -> bool {
        match self {
            TileType::Void => true,
            TileType::Floor => false,
            TileType::Wall => true,
            TileType::Hallway => false,
            TileType::Door(DoorType::Open) => false,
            TileType::Door(DoorType::Closed) => true,
            TileType::Door(DoorType::Archway) => false,
        }
    }
}
