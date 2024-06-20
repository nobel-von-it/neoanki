use crate::consts;
use core::fmt;
use std::io::stdout;

use crossterm::execute;

use crate::{
    data::Question,
    states::{InputCommands, State},
};

#[derive(Debug, Clone)]
pub struct Game {
    pub state: State,
    pub question: Question,
    pub translation: String,
    pub field: Field,
    pub result: String,
    pub score: usize,
}
impl Game {
    pub async fn start() -> Self {
        let state = State::Input;
        let question = Question::get_random();
        let translation = question.trans().await;
        let field = Field::new();
        let result = String::new();
        Self {
            state,
            question,
            translation,
            field,
            result,
            score: 0,
        }
    }
    pub async fn update(&mut self) {
        self.state = State::Game;
        self.question = Question::get_random();
        self.translation = self.question.trans().await;
        self.field = Field::new();
        self.result = String::new();
    }
    pub async fn check(&mut self) {}
    pub fn cmp(&self) -> Option<(String, bool)> {
        let right = self.question.answer.clone();
        let left = self.field.to_string();
        if left.len() == right.len() {
            // string with + or - symbols
            let mut res = String::new();
            let mut counter = 0;
            for i in 0..left.len() {
                if left.chars().nth(i).unwrap() == right.chars().nth(i).unwrap() {
                    res.push('+');
                    counter += 1;
                } else {
                    res.push('-');
                }
            }
            if counter == right.len() {
                return Some((res, true));
            }
            return Some((res, false));
        }
        // None means full incorrect answer
        None
    }
    pub async fn draw(&self) -> anyhow::Result<()> {
        // clear terminal and move cursor to top
        execute!(
            stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        )?;
        let (x, _y) = crossterm::terminal::size()?;
        match self.state {
            State::Input => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo((x / 2) - (x / 4), consts::QUESTION_POS)
                )?;
                println!("{}", consts::COMMAND_TEXT);

                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo((x / 2) - (x / 4), consts::QUESTION_POS + 1)
                )?;
                println!("{}", self.field.show());
            }
            State::Game => {
                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(
                        (x / 2) - (self.question.sentence.len() as u16 / 2),
                        consts::QUESTION_POS
                    )
                )?;
                println!("{}", self.question.sentence);

                // execute!(
                //     stdout(),
                //     crossterm::cursor::MoveTo((x / 2) - (self.translation.len() as u16 / 2), 3)
                // )?;
                // println!("{}", self.translation.trim());

                execute!(
                    stdout(),
                    crossterm::cursor::MoveTo(
                        (x / 2) - (self.field.text.len() as u16 / 2),
                        consts::QUESTION_POS + 3
                    ),
                )?;
                println!("{}", self.field.show());
            }
            State::QuestionManager => {}
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    pub text: Vec<char>,
    pub pos: usize,
}

impl Field {
    pub fn new() -> Self {
        Self {
            text: vec!['\0'],
            pos: 0,
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
                if i == self.pos {
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
        write!(f, "{}", self.text.iter().collect::<String>())
    }
}
