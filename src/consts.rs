pub const NAME: &str = "NEOANKI";
pub const VERSION: &str = "0.1.0";
pub const AUTHOR: &str = "Nerd";
pub const QUESTION_POS: u16 = 3;
pub const COMMAND_TEXT: &str = "Enter command:";
pub const WIN_SCORE: u16 = 5;

const DIR_NAMEL: &str = ".neoanki";
const DIR_NAMEW: &str = "Neoanki";

/// main languages
pub const LANGS: [&str; 3] = ["en", "de", "jp"];
pub const DEF_LANG: &str = "de";
pub const MY_LANG: &str = "ru";

pub fn dir_name() -> &'static str {
    match std::env::consts::OS {
        "windows" => DIR_NAMEW,
        _ => DIR_NAMEL,
    }
}

pub fn full_dir_name() -> String {
    match std::env::consts::OS {
        "windows" => format!("C:\\Users\\{}\\{}", whoami::username(), dir_name()),
        _ => format!("/home/{}/{}", whoami::username(), dir_name()),
    }
}

pub fn file_name(lang: &str) -> &'static str {
    match lang {
        "en" => "questions_en.json",
        "de" => "questions_de.json",
        "jp" => "questions_jp.json",
        _ => "questions_all.json",
    }
}

pub fn full_file_name(lang: &str) -> String {
    match std::env::consts::OS {
        "windows" => format!("{}\\{}", full_dir_name(), file_name(lang)),
        _ => format!("{}/{}", full_dir_name(), file_name(lang)),
    }
}
