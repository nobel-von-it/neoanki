#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    Game,
    Input,
    QuestionManager,
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
    pub fn to_string(&self) -> String {
        match self {
            InputCommands::Start => String::from("start"),
            InputCommands::Exit => String::from("exit"),
            InputCommands::Next => String::from("next"),
            InputCommands::Prev => String::from("prev"),
            InputCommands::Help => String::from("help"),
            InputCommands::Add => String::from("add"),
            InputCommands::Remove => String::from("remove"),
            InputCommands::Edit => String::from("edit"),
            InputCommands::None => String::from("none"),
        }
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Answer(String);
