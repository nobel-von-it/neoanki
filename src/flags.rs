use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Language
    #[arg(short, long, default_value_t = String::from("de"))]
    pub lang: String,
}

pub enum Lang {
    En,
    De,
    Jp,
}

impl Args {
    pub fn get_lang(&self) -> Lang {
        match self.lang.as_str() {
            "en" => Lang::En,
            "de" => Lang::De,
            "jp" => Lang::Jp,
            _ => Lang::De,
        }
    }
}
