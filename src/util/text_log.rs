use std::{
    fmt,
    fs::{self, File},
    io::{BufWriter, Write},
    path::PathBuf,
};

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
    file: Option<BufWriter<File>>,
}

impl Log {
    pub fn new(print_debug_info: bool) -> Self {
        let path = create_log_file();

        let file = File::create(path).ok();
        let writer = file.map(BufWriter::new);

        Self { print_debug_info, messages: Vec::new(), file: writer }
    }

    /// Specific getter that returns all messages, but filetered by debug messages or not depending on [Log::print_debug_info]
    pub fn get_messages_for_display(&self) -> Vec<&LogData> {
        if self.print_debug_info {
            self.messages.iter().collect()
        } else {
            self.messages
                .iter()
                .filter(|&message| {
                    !matches!(message, LogData::DebugInfo(_) | LogData::DebugWarn(_))
                })
                .collect()
        }
    }

    /// Add information about a new log event to the log.
    ///
    /// This is to be used as the primary way of logging.
    pub fn info(&mut self, log_data: LogData) {
        self.messages.push(log_data.clone());

        if let Some(file) = &mut self.file {
            let _ = writeln!(file, "{}", log_data);
        }
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
    pub fn debug_info(&mut self, message: String) {
        let lines: Vec<&str> = message.split("\n").collect();
        for line in lines {
            self.info(LogData::DebugInfo(line.to_string()));
        }
    }

    pub fn debug_warn(&mut self, message: String) {
        let lines: Vec<&str> = message.split("\n").collect();
        for line in lines {
            self.info(LogData::DebugWarn(line.to_string()));
        }
    }

    pub fn print_lore(&mut self) {
        self.info(LogData::Plain("It is written in the books of old:".into()));
        self.info(LogData::Lore("..The depths are like an anthill.".into()));
        self.info(LogData::Lore("..Dangerous. Ever-twisting. Dark.".into()));
        self.info(LogData::Lore("..Those who venture into this forsaken place".into()));
        self.info(LogData::Lore("..place must truly be mad.".into()));
        self.info(LogData::Lore("..And if they aren't, the anthill will make them.".into()));
        self.info(LogData::Lore("..May you find what you are looking for.".into()));
    }
}

/// Creates a log file in the OS's local data directory (./local/share on Linux)
/// The filename is timestamped
fn create_log_file() -> PathBuf {
    let mut path = dirs::data_local_dir().expect("No data directory found on this OS");
    path.push("Anthill");
    path.push("logs");
    fs::create_dir_all(&path).expect("Could not create data directory in OS.");

    let filename = format!("anthill_log_{}.txt", chrono::Local::now().format("%Y-%m-%d-%H-%M-%S"));
    path.push(filename);

    path
}

#[derive(Clone)]
pub enum LogData {
    DebugInfo(String),
    DebugWarn(String),
    Plain(String),
    Lore(String),
    PlayerAttackHit { npc_name: String, damage: u16 },
    PlayerAttackMiss { npc_name: String },
    PlayerEats { item_name: String },
    NpcAttackHit { npc_name: String, damage: u16 },
    NpcAttackMiss { npc_name: String },
    NpcDied { npc_name: String },
    InventoryFull,
    EquipmentSlotEmpty,
    UseStairsDown,
    UseStairsUp,
    NoInteraction,
    Overdose,
    PlayerHealed { amount: u16 },
    GauntletGreeting,
}

impl fmt::Display for LogData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogData::DebugInfo(_) => write!(f, "[ INFO ] {}", self.display()),
            LogData::DebugWarn(_) => write!(f, "[ WARN ] {}", self.display()),
            _ => write!(f, "         {}", self.display()),
        }
    }
}

impl LogData {
    pub fn display(&self) -> Line<'_> {
        match self {
            LogData::Plain(message) => Line::from(message.to_string()),
            LogData::DebugInfo(message) => Line::styled(message.to_string(), STYLE_DEBUG_INFO),
            LogData::DebugWarn(message) => Line::styled(message.to_string(), STYLE_DEBUG_WARN),
            LogData::Lore(message) => {
                Line::styled(message.to_string(), Style::new().add_modifier(Modifier::ITALIC))
            }
            LogData::PlayerAttackHit { npc_name, damage } => Line::from(vec![
                Span::styled("You", STYLE_YOU),
                Span::raw(" attack "),
                Span::styled(npc_name, STYLE_NPC),
                Span::raw(" and deal "),
                Span::styled(damage.to_string(), STYLE_NUMBER),
                Span::raw(" damage."),
            ]),
            LogData::PlayerAttackMiss { npc_name } => Line::from(vec![
                Span::styled("You", STYLE_YOU),
                Span::raw(" attack "),
                Span::styled(npc_name, STYLE_NPC),
                Span::raw(", but miss."),
            ]),
            LogData::PlayerEats { item_name } => Line::from(vec![
                Span::styled("You", STYLE_YOU),
                Span::raw(" eat "),
                Span::styled(item_name, STYLE_ITEM),
            ]),
            LogData::NpcAttackHit { npc_name, damage } => Line::from(vec![
                Span::styled(npc_name, STYLE_NPC),
                Span::raw(" attacks "),
                Span::styled("you", STYLE_YOU),
                Span::raw(" and deals "),
                Span::styled(damage.to_string(), STYLE_NUMBER),
                Span::raw(" damage."),
            ]),
            LogData::NpcAttackMiss { npc_name } => Line::from(vec![
                Span::styled(npc_name, STYLE_NPC),
                Span::raw(" attacks "),
                Span::styled("you", STYLE_YOU),
                Span::raw(", but misses."),
            ]),
            LogData::NpcDied { npc_name } => {
                Line::from(vec![Span::styled(npc_name, STYLE_NPC), Span::raw(" died.")])
            }
            LogData::InventoryFull => {
                Line::from("Your inventory is full. Cannot add another item.")
            }
            LogData::EquipmentSlotEmpty => {
                Line::from("The equipment slot is already empty. Cannot unequip.")
            }
            LogData::UseStairsDown => Line::from("You go down the stairs..."),
            LogData::UseStairsUp => Line::from("You go back up the stairs..."),
            LogData::NoInteraction => Line::from("You cannot interact with that object."),
            LogData::Overdose => Line::from("You are experiencing the effects of overdosing."),
            LogData::PlayerHealed { amount } => Line::from(vec![
                Span::raw("You regain "),
                Span::styled(amount.to_string(), STYLE_NUMBER),
                Span::raw(" hit points."),
            ]),
            LogData::GauntletGreeting => Line::from(vec![
                Span::styled("Welcome to the ", Style::new().add_modifier(Modifier::ITALIC)),
                Span::styled(
                    "Gauntlet",
                    Style::new()
                        .fg(Color::Red)
                        .add_modifier(Modifier::UNDERLINED)
                        .add_modifier(Modifier::ITALIC),
                ),
                Span::styled(". Prove your worth!", Style::new().add_modifier(Modifier::ITALIC)),
            ]),
        }
    }
}

// Pre-defined theme
const STYLE_DEBUG_INFO: Style = Style::new().fg(Color::DarkGray);
const STYLE_DEBUG_WARN: Style = Style::new().fg(Color::Red);
const STYLE_YOU: Style = Style::new().add_modifier(Modifier::ITALIC);
const STYLE_NPC: Style = Style::new().fg(Color::Yellow).add_modifier(Modifier::ITALIC);
const STYLE_ITEM: Style = Style::new().fg(Color::Magenta).add_modifier(Modifier::BOLD);
const STYLE_NUMBER: Style = Style::new().fg(Color::Cyan);
