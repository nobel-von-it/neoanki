use core::fmt;

use crate::states::InputCommands;

#[derive(Debug, Clone)]
pub struct Field {
    pub text: Vec<char>,
    pub pos: usize,
}

impl PartialEq for Field {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
impl Eq for Field {
    fn assert_receiver_is_total_eq(&self) {
        self.text.assert_receiver_is_total_eq();
    }
}
impl Field {
    pub fn new() -> Self {
        Self {
            text: vec!['\0'],
            pos: 0,
        }
    }
    pub fn from(text: &str) -> Self {
        let text = format!("\0{}", text);
        Self {
            text: text.chars().collect(),
            pos: text.len() - 1,
        }
    }
    pub fn left(&mut self) {
        if self.pos > 0 {
            self.pos -= 1
        }
    }
    pub fn right(&mut self) {
        if self.pos < self.text.len() - 1 && !self.text.is_empty() {
            self.pos += 1
        }
    }
    pub fn add(&mut self, c: char) {
        self.text.insert(self.pos + 1, c);
        self.right();
    }
    pub fn del(&mut self) {
        if self.text.len() > 1 && self.pos > 0 {
            self.text.remove(self.pos);
            self.left();
        }
    }
    pub fn get_command(&self) -> InputCommands {
        InputCommands::from_string(&self.to_string())
    }
    pub fn show(&self) -> String {
        self.text
            .iter()
            .enumerate()
            .map(|(i, c)| {
                if i == 0 {
                    String::new()
                } else if i == self.pos {
                    format!("{c}|")
                } else {
                    format!("{c}")
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }
}
impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text.iter().skip(1).collect::<String>())
    }
}
