use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    App,
    core::game::GameRules,
    data::{item_defs::item_defs, npc_defs::npc_defs},
    util::{
        errors_results::GameOutcome,
        rng::{Check, DieSize, Roll},
        text_log::LogData,
    },
    world::{coordinate_system::Point, tiles::Collision},
};

/// Different available commands in the game.
#[derive(Debug, EnumIter)]
pub enum GameCommand {
    /// Quits the game by closing the App. This does the same thing as pressing the quit button.
    ///
    /// # GameCommand Syntax
    /// `quit`
    Quit,

    /// Adds an item to the player character's inventory.
    ///
    /// # GameCommand Syntax
    /// `give <item_def> <amount>`
    /// * `item_def` - String of the `item_def_id`
    /// * `amount` - Number of items to give. Must be coercible into a `u32`!
    Give { item_def: String, amount: u32 },

    /// Displays all available commands and their descriptions.
    ///
    /// # GameCommand Syntax
    /// `help`
    Help,

    /// Gives the player character high statistics.
    ///
    /// # GameCommand Syntax
    /// `maxstats`
    MaxStats,

    /// Gives the player the best equipment in the game.
    ///
    /// # GameCommand Syntax
    /// `maxequip`
    MaxEquip,

    /// Prints player character debug info to the log.
    ///
    /// # GameCommand Syntax
    /// `playerinfo` or `pi`
    PlayerInfo,

    /// Prints rng debug info into the log.
    ///
    /// # GameCommand Syntax
    /// `rngtest`
    RngTest,

    /// Teleports the player.
    ///
    /// # GameCommand Syntax
    /// `teleport <x> <y>`
    /// * `x`/`y` - Coordinates to teleport to (must be coercible into a `usize`)
    Teleport(Point),

    /// Reduces player to 0 HP, resulting in the Game Over screen.
    ///
    /// # GameCommand Syntax
    /// `suicide`
    Suicide,

    /// Reveals all tiles on the map for 1 round.
    /// This also sets the exploration status of all tiles to `true`.
    ///
    /// # GameCommand Syntax
    /// `revealall`
    RevealAll,

    /// Displays a legend for every glyph in the log
    ///
    /// # GameCommand Syntax
    /// `legend`
    Legend,

    /// Toggles tile collision for the player character, allowing them to walk through walls.
    ///
    /// # GameCommand Syntax
    /// `noclip`
    NoClip,

    /// Toggles god mode for the player, making them immortal.
    ///
    /// # GameCommand Syntax
    /// `godmode`
    GodMode,
}

impl GameCommand {
    /// Returns the description of the command as displayed in-game using the 'help' command.
    pub fn description(&self) -> &'static str {
        match self {
            GameCommand::Quit => "Quit the game",
            GameCommand::Give { .. } => "Give an item to the player: `give <item def id> <amount>`",
            GameCommand::Help => "List available commands",
            GameCommand::MaxStats => "Grant max stats to player",
            GameCommand::MaxEquip => "Grant the best equipment to the player",
            GameCommand::PlayerInfo => "Print player info to log",
            GameCommand::RngTest => "Make a roll and a check to test the RNG Engine",
            GameCommand::Teleport(_) => {
                "Teleport the player to the given absolute position: `teleport <x> <y>`"
            }
            GameCommand::RevealAll => "Get vision over the entire map for 1 round",
            GameCommand::Suicide => "Set HP to zero to test game over state",
            GameCommand::Legend => "Show list of all map symbols",
            GameCommand::NoClip => "Toggle to walk through impassable terrain",
            GameCommand::GodMode => "Toggle invulnerability",
        }
    }

    /// Returns the name of the command as displayed in-game using the 'help' command.
    pub fn name(&self) -> &'static str {
        match self {
            GameCommand::Quit => "quit",
            GameCommand::Give { .. } => "give",
            GameCommand::Help => "help",
            GameCommand::MaxStats => "maxstats",
            GameCommand::MaxEquip => "maxequip",
            GameCommand::PlayerInfo => "playerinfo",
            GameCommand::RngTest => "rngtest",
            GameCommand::Teleport(_) => "teleport",
            GameCommand::Suicide => "suicide",
            GameCommand::RevealAll => "revealall",
            GameCommand::Legend => "legend",
            GameCommand::NoClip => "noclip",
            GameCommand::GodMode => "godmode",
        }
    }
}

impl TryFrom<String> for GameCommand {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut tokens = value.split_whitespace();

        let command = tokens.next().ok_or("No command given")?.to_lowercase();

        match command.as_str() {
            "quit" => Ok(GameCommand::Quit),
            "exit" => Ok(GameCommand::Quit),

            "help" => Ok(GameCommand::Help),

            "give" => {
                let item_def = tokens.next().ok_or("Missing item name")?.to_string();

                let amount =
                    tokens.next().and_then(|string| string.parse::<u32>().ok()).unwrap_or(1);

                Ok(GameCommand::Give { item_def, amount })
            }

            "maxstats" => Ok(GameCommand::MaxStats),
            "maxequip" => Ok(GameCommand::MaxEquip),

            "playerinfo" => Ok(GameCommand::PlayerInfo),
            "pi" => Ok(GameCommand::PlayerInfo),
            "rngtest" => Ok(GameCommand::RngTest),
            "teleport" => {
                let arg_x = tokens
                    .next()
                    .ok_or("Missing coordinates")?
                    .parse::<usize>()
                    .map_err(|_| "Invalid format for coordinates")?;
                let arg_y = tokens
                    .next()
                    .ok_or("Missing y-coordinate")?
                    .parse::<usize>()
                    .map_err(|_| "Invalid format for y-coordinate")?;

                Ok(GameCommand::Teleport(Point { x: arg_x, y: arg_y }))
            }
            "suicide" => Ok(GameCommand::Suicide),
            "revealall" => Ok(GameCommand::RevealAll),
            "legend" => Ok(GameCommand::Legend),
            "noclip" => Ok(GameCommand::NoClip),
            "godmode" => Ok(GameCommand::GodMode),
            _ => Err(format!("Unknown Command {}", command)),
        }
    }
}

impl App {
    /// Handles the execution of a given [`GameCommand`] in the `App` State.
    ///
    /// This is written into the app state, because it not only needs access to the `GameState`, but also some App functions.
    fn execute_command(&mut self, command: GameCommand) {
        match command {
            GameCommand::Quit => {
                self.should_quit = true;
            }
            GameCommand::Help => {
                for command in GameCommand::iter() {
                    self.game.log.print(format!(
                        "{:<12} - {}",
                        command.name(),
                        command.description(),
                    ))
                }
            }
            GameCommand::Give { item_def: item_def_id, amount } => {
                if self.game.get_item_def_by_id(&item_def_id).is_none() {
                    self.game.log.print("No item with this def_id exists.".to_string());
                    return;
                }

                let mut amount_given: u32 = 0;
                for _ in 0..amount {
                    if let Ok(item_id) = self.game.register_item(&item_def_id) {
                        match self.game.add_item_to_inv(item_id) {
                            Ok(GameOutcome::Success) => {
                                amount_given += 1;
                            }
                            _ => {
                                self.game.log.info(LogData::InventoryFull);
                                let _ = self.game.deregister_item(item_id);
                                break;
                            }
                        }
                    }
                }

                if amount_given > 0 {
                    self.game.log.print(format!(
                        "Added {} (x{}) to player's inventory",
                        item_def_id, amount
                    ));
                }
            }
            GameCommand::MaxStats => {
                self.game.player.character.stats.level = 100;
                self.game.player.character.stats.dexterity = 100;
                self.game.player.character.stats.perception = 100;
                self.game.player.character.stats.strength = 100;
                self.game.player.character.stats.vitality = 100;
                self.game.player.character.stats.base.hp_max = 500;
                self.game.player.character.stats.base.hp_current = 500;
                self.game.log.print("Advanced Player to Level 100.".to_string());
            }
            GameCommand::MaxEquip => {
                self.execute_command(GameCommand::Give {
                    item_def: "weapon_bow_cross".to_string(),
                    amount: 1,
                });
                self.execute_command(GameCommand::Give {
                    item_def: "weapon_warhammer".to_string(),
                    amount: 1,
                });
                self.execute_command(GameCommand::Give {
                    item_def: "armor_rustacean".to_string(),
                    amount: 1,
                });
                self.execute_command(GameCommand::Give {
                    item_def: "food_meat".to_string(),
                    amount: 5,
                });
                self.execute_command(GameCommand::Give {
                    item_def: "potion_healing_small".to_string(),
                    amount: 2,
                });
            }

            GameCommand::PlayerInfo => {
                self.game
                    .log
                    .print(format!("Character \"{}\"", self.game.player.character.base.name,));
                self.game.log.print(format!(
                    "-  HP: {}/{}",
                    self.game.player.character.stats.base.hp_current,
                    self.game.player.character.stats.base.hp_max,
                ));
                self.game
                    .log
                    .print(format!("-  Position: {}", self.game.player.character.base.pos,));
                self.game.log.print(format!(
                    "-  S:{}, D:{}, V:{}, P:{}",
                    self.game.player.character.stats.strength,
                    self.game.player.character.stats.dexterity,
                    self.game.player.character.stats.vitality,
                    self.game.player.character.stats.perception,
                ));
                self.game.log.print(format!(
                    "-  Dodge:{}, Dmg:{}/{}",
                    self.game.player.character.dodge_chance(),
                    self.game.player.character.attack_damage_bonus_melee(),
                    self.game.player.character.attack_damage_bonus_ranged(),
                ));
            }

            GameCommand::RngTest => {
                let roll: i16 = self.game.roll(&Roll::new(1, DieSize::D6));
                let check: bool = self.game.check(&Check::default().set_difficulty(10));
                self.game.log.print(format!(
                    "Rolling 1d6: {:?}\nChecking 1d20 against difficulty 10: {:?}",
                    roll, check,
                ))
            }

            GameCommand::Teleport(point) => {
                if !self.game.current_world().is_in_bounds(point.x as isize, point.y as isize) {
                    self.game.log.print(format!("Position {} is out of bounds", point));
                    return;
                }

                if !self.game.current_world().get_tile(point).tile_type.is_walkable() {
                    self.game.log.print(format!("Position {} cannot be occupied by player", point));
                    return;
                }

                self.game.player.character.base.pos = point;
                self.game.log.print("You were teleported.".to_string());
            }

            GameCommand::Suicide => {
                self.game.log.print("Player committed suicide".to_string());
                self.game.player.character.stats.base.hp_current = 0;
            }

            GameCommand::RevealAll => {
                self.game.log.print("Revealing all tiles.".to_string());

                for tile in self.game.current_world_mut().tiles.iter_mut() {
                    tile.make_visible();
                    tile.make_explored();
                }
            }

            GameCommand::Legend => {
                self.game.log.print("@ - Player Character (you)".to_string());
                self.game.log.print("+ - Door (closed)".to_string());
                self.game.log.print("_ - Door (open)".to_string());
                for item in item_defs().values() {
                    self.game.log.print(format!("{} - {}", item.glyph, item.name));
                }
                for npc in npc_defs().values() {
                    self.game.log.print(format!("{} - {}", npc.glyph, npc.name));
                }
            }

            GameCommand::NoClip => {
                self.game.game_rules.toggle(GameRules::NO_CLIP);
                self.game.log.print("Toggled No-Clip Mode.".to_string());
            }

            GameCommand::GodMode => {
                self.game.game_rules.toggle(GameRules::GOD_MODE);
                self.game.log.print("Toggled God Mode.".to_string());
            }
        }
    }

    /// Tries to run a [`GameCommand`] from the string that was input by the user.
    ///
    /// If the String matches an available command, it is executed.
    pub fn run_command(&mut self, input: String) {
        match GameCommand::try_from(input) {
            Ok(command) => self.execute_command(command),
            Err(error) => self.game.log.print(error),
        }
    }
}
