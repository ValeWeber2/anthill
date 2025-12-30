use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    App,
    util::rng::{Check, DieSize, Roll},
    world::worldspace::Collision,
};

#[derive(Debug, EnumIter)]
pub enum Command {
    Quit,
    Give { item_def: String, amount: u32 },
    Help,
    MaxStats,
    MaxEquip,
    PlayerInfo,
    RngTest,
    Teleport { x: usize, y: usize },
}

impl Command {
    pub fn description(&self) -> &'static str {
        match self {
            Command::Quit => "Quit the game.",
            Command::Give { .. } => {
                "Give an item to the player. Usage `give <item def id> <amount>`."
            }
            Command::Help => "List available commands.",
            Command::MaxStats => "Grants max stats to player.",
            Command::MaxEquip => "Grants the best equipment to the player.",
            Command::PlayerInfo => "Prints player info to log.",
            Command::RngTest => "Makes a roll and a check to test the RNG Engine",
            Command::Teleport { .. } => "Teleports the player to the given absolute position",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Command::Quit => "quit",
            Command::Give { .. } => "give",
            Command::Help => "help",
            Command::MaxStats => "maxstats",
            Command::MaxEquip => "maxequip",
            Command::PlayerInfo => "playerinfo",
            Command::RngTest => "rngtest",
            Command::Teleport { .. } => "teleport",
        }
    }
}

pub fn parse_command(input: &str) -> Result<Command, String> {
    let mut tokens = input.split_whitespace();

    let command = tokens.next().ok_or("No command given")?.to_lowercase();

    match command.as_str() {
        "quit" => Ok(Command::Quit),
        "exit" => Ok(Command::Quit),

        "help" => Ok(Command::Help),

        "give" => {
            let item_def = tokens.next().ok_or("Missing item name")?.to_string();

            let amount = tokens.next().ok_or("Missing item amount")?.parse::<u32>().unwrap_or(1);

            Ok(Command::Give { item_def, amount })
        }

        "maxstats" => Ok(Command::MaxStats),
        "maxequip" => Ok(Command::MaxEquip),

        "playerinfo" => Ok(Command::PlayerInfo),
        "pi" => Ok(Command::PlayerInfo),
        "rngtest" => Ok(Command::RngTest),
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

            Ok(Command::Teleport { x: arg_x, y: arg_y })
        }
        _ => Err(format!("Unknown Command {}", command)),
    }
}

impl App {
    fn execute_command(&mut self, command: Command) {
        match command {
            Command::Quit => {
                self.should_quit = true;
            }
            Command::Help => {
                for command in Command::iter() {
                    self.game.log.print(format!(
                        "{:<12} - {}",
                        command.name(),
                        command.description(),
                    ))
                }
            }
            Command::Give { item_def, amount } => {
                self.game.log.print(format!("Added {} {} to player's inventory", item_def, amount));
                todo!("Implement a cheat to give player character an item.");
            }
            Command::MaxStats => todo!("Implement once player logic is finished"),
            Command::MaxEquip => todo!("Implement once items are finished"),

            Command::PlayerInfo => {
                self.game.log.print(format!(
                    "Character \"{}\"\n-  HP: {}/{}\n-  Position: x: {}, y: {}",
                    self.game.player.character.base.name,
                    self.game.player.character.stats.base.hp_current,
                    self.game.player.character.stats.base.hp_max,
                    self.game.player.character.base.pos.x,
                    self.game.player.character.base.pos.y,
                ));
            }

            Command::RngTest => {
                let roll: i16 = self.game.roll(&Roll::new(1, DieSize::D6));
                let check: bool = self.game.check(&Check::default().set_difficulty(10));
                self.game.log.print(format!(
                    "Rolling 1d6: {:?}\nChecking 1d20 against difficulty 10: {:?}",
                    roll, check,
                ))
            }

            Command::Teleport { x, y } => {
                if !self.game.world.get_tile(x, y).tile_type.is_walkable() {
                    self.game
                        .log
                        .print(format!("Position {} {} cannot be occupied by player", x, y));
                    return;
                }

                if !self.game.world.is_in_bounds(x as isize, y as isize) {
                    self.game.log.print(format!("Position {} {} is out of bounds", x, y));
                    return;
                }

                self.game.player.character.base.pos.x = x;
                self.game.player.character.base.pos.y = y;
            }
        }
    }

    pub fn run_command(&mut self, input: String) {
        match parse_command(&input) {
            Ok(command) => self.execute_command(command),
            Err(error) => self.game.log.print(error),
        }
    }
}
