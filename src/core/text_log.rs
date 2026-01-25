use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
};

/// The game's text log. The events of the game are desribed for the user in the log.
///
/// The log can also be used to display debug messages.
pub struct Log {
    pub print_debug_info: bool,
    pub messages: Vec<LogData>,
}

impl Log {
    pub fn new(print_debug_info: bool) -> Self {
        Self { print_debug_info, messages: Vec::new() }
    }

    /// Add information about a new log event to the log.
    ///
    /// This is to be used as the primary way of logging.
    pub fn info(&mut self, log_data: LogData) {
        self.messages.push(log_data);
    }

    /// Add plain text to the log.
    ///
    /// # Note
    /// Use sparingly. It is better to use [LogData] and [Log::info] to log information in a data-driven way.
    pub fn print(&mut self, message: String) {
        let lines: Vec<&str> = message.split("\n").collect();
        for line in lines {
            self.info(LogData::Plain(line.to_string()));
        }
    }

    /// Add plain text to the log as long as [Log::print_debug_info] is true.
    ///
    /// Use this for printing debug and development information to the log.
    pub fn debug_print(&mut self, message: String) {
        if !self.print_debug_info {
            return;
        }

        let lines: Vec<&str> = message.split("\n").collect();
        for line in lines {
            self.info(LogData::Plain(line.to_string()));
        }
    }
}

pub enum LogData {
    Debug(String),
    Plain(String),
    PlayerAttackHit { npc_name: String, damage: u16 },
    PlayerAttackMiss { npc_name: String },
    PlayerEats { item_name: String },
    NpcAttackHit { npc_name: String, damage: u16 },
    NpcAttackMiss { npc_name: String },
    NpcDied { npc_name: String },
    InventoryFull,
    CannotUnequipEmptySlot,
}

impl LogData {
    pub fn display(&self) -> Line {
        match self {
            LogData::Plain(message) => Line::from(message.to_string()),

            LogData::Debug(message) => Line::styled(message.to_string(), STYLE_DEBUG),

            LogData::PlayerAttackHit { npc_name, damage } => Line::from(vec![
                Span::styled("You", STYLE_YOU),
                Span::raw(" attack the "),
                Span::styled(npc_name, STYLE_NPC),
                Span::raw(" and deal "),
                Span::styled(damage.to_string(), STYLE_NUMBER),
                Span::raw(" damage."),
            ]),

            LogData::PlayerAttackMiss { npc_name } => Line::from(vec![
                Span::styled("You", STYLE_YOU),
                Span::raw(" attack the "),
                Span::styled(npc_name, STYLE_NPC),
                Span::raw(", but miss."),
            ]),

            LogData::PlayerEats { item_name } => Line::from(vec![
                Span::styled("You", STYLE_YOU),
                Span::raw(" eat the "),
                Span::styled(item_name, STYLE_ITEM),
            ]),

            LogData::NpcAttackHit { npc_name, damage } => Line::from(vec![
                Span::raw("The "),
                Span::styled(npc_name, STYLE_NPC),
                Span::raw(" attacks "),
                Span::styled("you", STYLE_YOU),
                Span::raw(" and deals "),
                Span::styled(damage.to_string(), STYLE_NUMBER),
                Span::raw(" damage."),
            ]),

            LogData::NpcAttackMiss { npc_name } => Line::from(vec![
                Span::raw("The "),
                Span::styled(npc_name, STYLE_NPC),
                Span::raw(" attacks "),
                Span::styled("you", STYLE_YOU),
                Span::raw(", but misses."),
            ]),

            LogData::NpcDied { npc_name } => Line::from(vec![
                Span::raw("The "),
                Span::styled(npc_name, STYLE_NPC),
                Span::raw(" died."),
            ]),

            LogData::InventoryFull => {
                Line::from("Your inventory is full. Cannot add another item.")
            }

            LogData::CannotUnequipEmptySlot => {
                Line::from("The equipment slot is already empty. Cannot unequip.")
            }
        }
    }
}

// Pre-defined theme
const STYLE_DEBUG: Style = Style::new().fg(Color::DarkGray);
const STYLE_YOU: Style = Style::new().add_modifier(Modifier::ITALIC);
const STYLE_NPC: Style = Style::new().fg(Color::Red).add_modifier(Modifier::ITALIC);
const STYLE_ITEM: Style = Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD);
const STYLE_NUMBER: Style = Style::new().fg(Color::Cyan).add_modifier(Modifier::UNDERLINED);
