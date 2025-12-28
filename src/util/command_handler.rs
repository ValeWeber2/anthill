use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    App,
    util::rng::{Check, DieSize, Roll},
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
                self.game
                    .log
                    .messages
                    .push(format!("Added {} {} to player's inventory", item_def, amount));
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
        }
    }

    pub fn run_command(&mut self, input: String) {
        match parse_command(&input) {
            Ok(command) => self.execute_command(command),
            Err(error) => self.game.log.print(error),
        }
    }
}
