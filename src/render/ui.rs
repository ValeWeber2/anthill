#![allow(dead_code)]

use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

use crate::{
    App,
    core::{
        game::{EntityBase, ItemSprite, Npc},
        player::PlayerCharacter,
    },
    render::menu_display::{Menu, MenuData},
    world::{
        tiles::{Tile, TileType},
        worldspace::{Drawable, Point, World},
    },
};

use crate::world::worldspace::{WORLD_HEIGHT, WORLD_WIDTH};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let world_width_u16: u16 = WORLD_WIDTH.try_into().unwrap();
        let world_height_u16: u16 = WORLD_HEIGHT.try_into().unwrap();

        let layout_top_bottom = Layout::vertical([Constraint::Min(0), Constraint::Length(4)]);
        let [area_game, area_character] = layout_top_bottom.areas(area);

        let layout_left_right = Layout::horizontal([
            Constraint::Percentage(70),
            Constraint::Length(1),
            Constraint::Percentage(30),
        ]);
        let [area_world, _empty, area_menu] = layout_left_right.areas(area_game);

        let outer_width = world_width_u16 + 2;
        let outer_height = world_height_u16 + 2;
        let area_worldspace = Layout::vertical([Constraint::Length(outer_height)])
            .horizontal_margin((area_world.width.saturating_sub(outer_width)) / 2)
            .vertical_margin((area_world.height.saturating_sub(outer_height)) / 2)
            .split(area_world)[0];

        // Character Info
        let block_character = Block::default().title("Character Info").borders(Borders::ALL);
        block_character.render(area_character, buf);

        // World
        let block_world = Block::default().title("World").borders(Borders::ALL);
        block_world.render(area_world, buf);

        // World Space
        // (Space actually occupied by tiles)
        let block_world = Block::default().title("World Space").borders(Borders::ALL);
        let block_world_inner = block_world.inner(area_worldspace);
        block_world.render(area_worldspace, buf);

        // Z-layer 0
        self.ui.world_display.render(&self.game.world, block_world_inner, buf);
        // Z-layer 1
        self.ui.world_display.render_items(&self.game.world.items, block_world_inner, buf);
        // Z-layer 2
        self.ui.world_display.render_npcs(&self.game.world.npcs, block_world_inner, buf);
        // Z-layer 3
        self.ui.world_display.render_player(&self.game.player.character, block_world_inner, buf);

        // Menu (Log, menus, tables)
        let block_menu = Block::default().title("Menu").borders(Borders::ALL);
        let block_menu_inner = block_menu.inner(area_menu);
        block_menu.render(area_menu, buf);

        self.ui.menu.render(
            MenuData {
                log: &self.game.log.messages,
                inventory: &[], // TODO Inventory
            },
            block_menu_inner,
            buf,
        );
    }
}

pub struct UserInterface {
    pub menu: Menu,
    pub world_display: WorldDisplay,
}

impl UserInterface {
    pub fn new() -> Self {
        Self { menu: Menu::new(), world_display: WorldDisplay {} }
    }
}

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

    fn render_player(&self, pc: &PlayerCharacter, rect: Rect, buf: &mut Buffer) {
        self.render_sprite(&pc.base, rect, buf);
    }

    fn render_npcs(&self, npcs: &Vec<Npc>, rect: Rect, buf: &mut Buffer) {
        for npc in npcs {
            self.render_sprite(&npc.base, rect, buf);
        }
    }

    fn render_items(&self, item_sprites: &Vec<ItemSprite>, rect: Rect, buf: &mut Buffer) {
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
    // match mask {
    //     NORTH | SOUTH => '│', // 0011 -> 3
    //     EAST | WEST   => '─', // 1100 -> 12
    //     NORTH | EAST  => '└', // 1001 -> 9
    //     NORTH | WEST  => '┘', // 0101 -> 5
    //     SOUTH | EAST  => '┌', // 1010 -> 10
    //     SOUTH | WEST  => '┐', // 0110 -> 6
    //     NORTH | SOUTH | EAST => '├', // 1011 -> 11
    //     NORTH | SOUTH | WEST => '┤', // 0111 -> 7
    //     EAST | WEST | NORTH  => '┴', // 1011 -> 13
    //     EAST | WEST | SOUTH  => '┬', // 1110 -> 14
    //     NORTH | SOUTH | EAST | WEST => '┼', // 1111-> 15
    //     _ => '#', // otherwise
    // }
    if mask == NORTH | SOUTH {
        '│'
    }
    // 0011 -> 3
    else if mask == EAST | WEST {
        '─'
    }
    // 1100 -> 12
    else if mask == NORTH | EAST {
        '└'
    }
    // 1001 -> 9
    else if mask == NORTH | WEST {
        '┘'
    }
    // 0101 -> 5
    else if mask == SOUTH | EAST {
        '┌'
    }
    // 1010 -> 10
    else if mask == SOUTH | WEST {
        '┐'
    }
    // 0110 -> 6
    else if mask == NORTH | SOUTH | EAST {
        '├'
    }
    // 1011 -> 11
    else if mask == NORTH | SOUTH | WEST {
        '┤'
    }
    // 0111 -> 7
    else if mask == EAST | WEST | NORTH {
        '┴'
    }
    // 1011 -> 13
    else if mask == EAST | WEST | SOUTH {
        '┬'
    }
    // 1110 -> 14
    else if mask == NORTH | SOUTH | EAST | WEST {
        '┼'
    }
    // 1111-> 15
    else {
        return '#';
    } // otherwise
}
