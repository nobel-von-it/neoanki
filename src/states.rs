use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Game,
    Check,
    Input,
    QuestionManager,
    Win,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            State::Game => write!(f, "Game"),
            State::Check => write!(f, "Check"),
            State::Input => write!(f, "Input"),
            State::QuestionManager => write!(f, "QuestionManager"),
            State::Win => write!(f, "Win"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputCommands {
    Start,
    Exit,
    Next,
    Prev,
    Help,
    Add,
    Remove,
    Edit,
    None,
}
impl InputCommands {
    pub fn from_string(s: &str) -> Self {
        match s {
            "start" | "begin" | "new" | "continue" => InputCommands::Start,
            "exit" | "quit" | "stop" | "end" => InputCommands::Exit,
            "next" | "skip" => InputCommands::Next,
            "prev" | "previous" | "back" | "undo" => InputCommands::Prev,
            "help" | "?" => InputCommands::Help,
            "add" | "create" | "insert" | "push" => InputCommands::Add,
            "remove" | "delete" | "drop" => InputCommands::Remove,
            "edit" | "update" | "change" | "modify" => InputCommands::Edit,
            _ => InputCommands::None,
        }
    }
}
impl fmt::Display for InputCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InputCommands::Start => write!(f, "start"),
            InputCommands::Exit => write!(f, "exit"),
            InputCommands::Next => write!(f, "next"),
            InputCommands::Prev => write!(f, "prev"),
            InputCommands::Help => write!(f, "help"),
            InputCommands::Add => write!(f, "add"),
            InputCommands::Remove => write!(f, "remove"),
            InputCommands::Edit => write!(f, "edit"),
            InputCommands::None => write!(f, "none"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer(String);
