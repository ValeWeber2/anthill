use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    App,
    core::text_log::LogData,
    util::{
        errors_results::GameOutcome,
        rng::{Check, DieSize, Roll},
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
    /// suicide
    Suicide,

    /// Reveals all tiles on the map for 1 round.
    /// This also sets the exploration status of all tiles to `true`.
    ///
    /// # GameCommand Syntax
    /// revealall
    RevealAll,
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
                    tokens.next().ok_or("Missing item amount")?.parse::<u32>().unwrap_or(1);

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
                    .map_err(|_| "Invalid format for coordinates.")?;
                let arg_y = tokens
                    .next()
                    .ok_or("Missing y-coordinate")?
                    .parse::<usize>()
                    .map_err(|_| "Invalid format for y-coordinate.")?;

                Ok(GameCommand::Teleport(Point { x: arg_x, y: arg_y }))
            }
            "suicide" => Ok(GameCommand::Suicide),
            "revealall" => Ok(GameCommand::RevealAll),
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
            GameCommand::Give { item_def, amount } => {
                for _ in 0..amount {
                    let item_id = self.game.register_item(item_def.clone());
                    match self.game.add_item_to_inv(item_id) {
                        Ok(GameOutcome::Success) => self
                            .game
                            .log
                            .print(format!("Added {} {} to player's inventory", item_def, amount)),
                        _ => {
                            self.game.log.info(LogData::InventoryFull);
                            let _ = self.game.deregister_item(item_id);
                            break;
                        }
                    }
                }
            }
            GameCommand::MaxStats => {
                self.game.player.character.stats.dexterity = 100;
                self.game.player.character.stats.perception = 100;
                self.game.player.character.stats.strength = 100;
                self.game.player.character.stats.vitality = 100;
                self.game.player.character.stats.base.hp_max = 500;
                self.game.player.character.stats.base.hp_current = 500;
            }
            GameCommand::MaxEquip => todo!("Implement once items are finished"),

            GameCommand::PlayerInfo => {
                self.game.log.print(format!(
                    "Character \"{}\"\n-  HP: {}/{}\n-  Position: x: {}, y: {}\n-  S:{}, D:{}, V:{}, P:{}",
                    self.game.player.character.base.name,
                    self.game.player.character.stats.base.hp_current,
                    self.game.player.character.stats.base.hp_max,
                    self.game.player.character.base.pos.x,
                    self.game.player.character.base.pos.y,
                    self.game.player.character.stats.dexterity,
                    self.game.player.character.stats.perception,
                    self.game.player.character.stats.strength,
                    self.game.player.character.stats.vitality,
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

            GameCommand::Teleport(pos) => {
                if !self.game.world.get_tile(pos).tile_type.is_walkable() {
                    self.game.log.print(format!("Position {} cannot be occupied by player", pos));
                    return;
                }

                if !self.game.world.is_in_bounds(pos.x as isize, pos.y as isize) {
                    self.game.log.print(format!("Position {} is out of bounds", pos));
                    return;
                }

                self.game.player.character.base.pos = pos;
            }

            GameCommand::Suicide => {
                self.game.player.character.stats.base.hp_current = 0;
            }

            GameCommand::RevealAll => {
                self.game.log.print("Revealing all Tiles".to_string());

                for tile in self.game.world.tiles.iter_mut() {
                    tile.make_visible();
                    tile.make_explored();
                }
            }
        }
    }

    /// Tries to run a [`GameCommand`] from the string that was input by the user.
    ///
    /// If the String matches an available command, it is executed.
    pub fn run_command(&mut self, input: String) {
        match GameCommand::try_from(input) {
            Ok(command) => self.execute_command(command),
            Err(error) => self.game.log.debug_print(error),
        }
    }
}
