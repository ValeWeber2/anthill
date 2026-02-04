use ratatui::prelude::*;

use crate::{
    core::{
        entity_logic::{Entity, EntityBase},
        game::GameState,
        player::PlayerCharacter,
    },
    world::{
        coordinate_system::{Direction, Point},
        tiles::{Drawable, Tile, TileType},
        worldspace::World,
    },
};

pub struct WorldDisplay;

impl WorldDisplay {
    /// Main function to display the worldspace
    ///
    /// Renders every tile of the worldspace by placing the characer manually.
    /// * Skips invisible and unexplored tiles
    /// * Applies conditional rendering to walls so they connect
    /// * Renders invisible explored tiles in gray
    pub fn render(&self, game: &GameState, rect: Rect, buf: &mut Buffer) {
        for y in 0..game.current_world().height {
            for x in 0..game.current_world().width {
                let point: Point = Point { x, y };
                let tile: &Tile = game.current_world().get_tile(point);

                // Skip invisible and unexplored tiles
                if !tile.visible && !tile.explored {
                    continue;
                }

                // Display coordinates
                let (display_x, display_y) = get_world_display_pos(point, rect);
                // Cell on the terminal canvas
                let cell: Option<&mut buffer::Cell> =
                    buf.cell_mut(Position::new(display_x, display_y));

                if let Some(cell_content) = cell {
                    // Walls are a special case due to their conditional rendering (wall mask)
                    if tile.tile_type == TileType::Wall {
                        let mask = wall_mask(game.current_world(), point);
                        cell_content.set_char(wall_glyph(mask));
                    } else {
                        cell_content.set_char(tile.tile_type.glyph());
                    }

                    // Invisible explored tiles are styled in a shade of grey, others normally
                    if !tile.visible && tile.explored {
                        cell_content.set_style(Style::default().fg(Color::DarkGray));
                    } else {
                        cell_content.set_style(tile.tile_type.style());
                    }
                }
            }
        }
    }

    /// Renders the player character at their own position in the world.
    pub fn render_player(&self, pc: &PlayerCharacter, rect: Rect, buf: &mut Buffer) {
        self.render_sprite(&pc.base, rect, buf);
    }

    /// Renders all Npcs at their position in the world.
    pub fn render_npcs(&self, game: &GameState, rect: Rect, buf: &mut Buffer) {
        for npc in &game.current_level().npcs {
            if game.current_world().get_tile(npc.pos()).visible {
                self.render_sprite(&npc.base, rect, buf);
            }
        }
    }

    /// Renders all Items at their position in the world.
    pub fn render_items(&self, game: &GameState, rect: Rect, buf: &mut Buffer) {
        for item_sprite in &game.current_level().item_sprites {
            if game.current_world().get_tile(item_sprite.pos()).visible {
                self.render_sprite(&item_sprite.base, rect, buf);
            }
        }
    }

    /// Renders a sprite (a single, dynamic character) on top of the worldspace.
    ///
    /// Can be used to render items, npcs, and the player character.
    fn render_sprite(&self, entity_base: &EntityBase, rect: Rect, buf: &mut Buffer) {
        let (display_x, display_y) = get_world_display_pos(entity_base.pos, rect);
        let cell = buf.cell_mut(Position::new(display_x, display_y));

        if let Some(cell_content) = cell {
            cell_content.set_char(entity_base.glyph());
            cell_content.set_style(entity_base.style());
        }
    }
}

/// Helper function to translate a position in the world into actual coordinates of characters in the Terminal screen.
#[inline]
pub fn get_world_display_pos(pos: Point, rect: Rect) -> (u16, u16) {
    (rect.x + pos.x as u16, rect.y + pos.y as u16)
}

// Conditional Wall Rendering

/// Bitmask, defining that a wall can be found to the north of the given position.
const NORTH: u8 = 1 << 0; // 0001 -> 1
/// Bitmask, defining that a wall can be found to the south of the given position.
const SOUTH: u8 = 1 << 1; // 0010 -> 2
/// Bitmask, defining that a wall can be found to the east of the given position.
const WEST: u8 = 1 << 2; // 0100 -> 4
/// Bitmask, defining that a wall can be found to the west of the given position.
const EAST: u8 = 1 << 3; // 1000 -> 8

/// Helper function that takes a position of a wall tile and calculates a wall mask for it.
///
/// # Returns
/// This function returns a `u8` number, where the four least significant bits represent whether a wall tile is at one of the neighbouring points.
///
/// The result consists of the following bits: `X X X X E W S N` (X=empty, N=North, S=South, E=East, W=West)
///
/// If a bit at the given position is 1, then that means there's a wall tile neighbouring the given tile in the given direction.
fn wall_mask(world: &World, point: Point) -> u8 {
    let mut mask = 0;

    if matches!(world.get_tile(point + Direction::Up).tile_type, TileType::Wall | TileType::Door(_))
    {
        mask |= NORTH; // +0001 -> +1
    }
    if matches!(
        world.get_tile(point + Direction::Down).tile_type,
        TileType::Wall | TileType::Door(_)
    ) {
        mask |= SOUTH; // +0010 -> +2
    }
    if matches!(
        world.get_tile(point + Direction::Left).tile_type,
        TileType::Wall | TileType::Door(_)
    ) {
        mask |= WEST; // +0100 -> +4
    }
    if matches!(
        world.get_tile(point + Direction::Right).tile_type,
        TileType::Wall | TileType::Door(_)
    ) {
        mask |= EAST; // +1000 -> +8
    }

    mask
}

/// Translates a wall mask (`u8`), created by [wall_mask], into an unicode character with the correct orientation that connects to adjacent wall tiles.
///
/// # Returns
/// Returns a glyph (`char`) to render as the wall tile.
fn wall_glyph(mask: u8) -> char {
    if mask == NORTH | SOUTH {
        '│' // 0011 -> 3
    } else if mask == EAST | WEST {
        '─' // 1100 -> 12
    } else if mask == NORTH | EAST {
        '└' // 1001 -> 9
    } else if mask == NORTH | WEST {
        '┘' // 0101 -> 5
    } else if mask == SOUTH | EAST {
        '┌' // 1010 -> 10
    } else if mask == SOUTH | WEST {
        '┐' // 0110 -> 6
    } else if mask == NORTH | SOUTH | EAST {
        '├' // 1011 -> 11
    } else if mask == NORTH | SOUTH | WEST {
        '┤' // 0111 -> 7
    } else if mask == EAST | WEST | NORTH {
        '┴' // 1011 -> 13
    } else if mask == EAST | WEST | SOUTH {
        '┬' // 1110 -> 14
    } else if mask == NORTH | SOUTH | EAST | WEST {
        '┼' // 1111-> 15
    } else if mask == NORTH || mask == SOUTH {
        '│'
    } else if mask == EAST || mask == WEST {
        '─'
    } else {
        '│'
    }
}
