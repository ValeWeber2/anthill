use ratatui::prelude::*;

use crate::{
    core::{
        entity_logic::{EntityBase, Npc},
        game_items::GameItemSprite,
        player::PlayerCharacter,
    },
    world::{
        tiles::{Tile, TileType},
        worldspace::{Drawable, Point, World},
    },
};

pub struct WorldDisplay;

impl WorldDisplay {
    pub fn render(&self, world: &World, rect: Rect, buf: &mut Buffer) {
        for y in 0..world.height {
            for x in 0..world.width {
                let tile: &Tile = world.get_tile(x, y);
                let (display_x, display_y) = get_world_display_pos(x, y, rect);
                let cell: Option<&mut buffer::Cell> =
                    buf.cell_mut(Position::new(display_x, display_y));

                if let Some(cell_content) = cell {
                    if tile.tile_type == TileType::Wall {
                        let mask = wall_mask(world, x, y);
                        cell_content.set_char(wall_glyph(mask));
                    } else {
                        cell_content.set_char(tile.tile_type.glyph());
                    }
                    cell_content.set_style(tile.tile_type.style());
                }
            }
        }
    }

    pub fn render_player(&self, pc: &PlayerCharacter, rect: Rect, buf: &mut Buffer) {
        self.render_sprite(&pc.base, rect, buf);
    }

    pub fn render_npcs(&self, npcs: &Vec<Npc>, rect: Rect, buf: &mut Buffer) {
        for npc in npcs {
            self.render_sprite(&npc.base, rect, buf);
        }
    }

    pub fn render_items(&self, item_sprites: &Vec<GameItemSprite>, rect: Rect, buf: &mut Buffer) {
        for item_sprite in item_sprites {
            self.render_sprite(&item_sprite.base, rect, buf);
        }
    }

    fn render_sprite(&self, entity_base: &EntityBase, rect: Rect, buf: &mut Buffer) {
        let Point { x, y } = entity_base.pos;
        let (display_x, display_y) = get_world_display_pos(x, y, rect);
        let cell = buf.cell_mut(Position::new(display_x, display_y));

        if let Some(cell_content) = cell {
            cell_content.set_char(entity_base.glyph());
            cell_content.set_style(entity_base.style());
        }
    }
}

#[inline]
pub fn get_world_display_pos(x: usize, y: usize, rect: Rect) -> (u16, u16) {
    (rect.x + x as u16, rect.y + y as u16)
}

// Conditional Wall Rendering
const NORTH: u8 = 1 << 0; // 0001 -> 1
const SOUTH: u8 = 1 << 1; // 0010 -> 2
const WEST: u8 = 1 << 2; // 0100 -> 4
const EAST: u8 = 1 << 3; // 1000 -> 8

fn wall_mask(world: &World, x: usize, y: usize) -> u8 {
    let mut mask = 0;

    if world.get_tile(x, y.saturating_sub(1)).tile_type == TileType::Wall {
        mask |= NORTH; // +0001 -> +1
    }
    if world.get_tile(x, y + 1).tile_type == TileType::Wall {
        mask |= SOUTH; // +0010 -> +2
    }
    if world.get_tile(x.saturating_sub(1), y).tile_type == TileType::Wall {
        mask |= WEST; // +0100 -> +4
    }
    if world.get_tile(x + 1, y).tile_type == TileType::Wall {
        mask |= EAST; // +1000 -> +8
    }

    mask
}

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
    } else {
        '#' // otherwise
    }
}
