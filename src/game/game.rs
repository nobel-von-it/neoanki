use log::info;

use crate::{consts, data::Question, states::State};

use super::field::Field;

#[derive(Debug, Clone)]
pub struct Game {
    pub state: State,
    pub dump: Dump,
    pub question: Question,
    pub translation: String,
    pub field: Field,
    pub result: u16,
    pub score: u16,
}
impl Game {
    pub fn start() -> Self {
        let state = State::Input;
        let dump = Dump::new();
        let question = Question::default();
        let translation = String::new();
        let field = Field::new();
        Self {
            state,
            dump,
            question,
            translation,
            field,
            result: 0,
            score: 0,
        }
    }
    pub fn switch(&mut self) {
        // if self.state == State::Input && self.dump == State::Input {
        //     return;
        // }
        // if self.state == State::Input {
        //     self.state = self.dump.clone();
        //     return;
        // }
        // self.dump = self.state.clone();
        // self.state = State::Input;
        info!("In switch");
        info!(
            "Now state is {:?}, dump is {:?}",
            self.state, self.dump.state
        );
        if self.state == State::Input && self.dump.state == State::Input {
            return;
        }
        if self.state == State::Input {
            self.state = self.dump.state.clone();
            self.field = Field::from(&self.dump.field.to_string());
            return;
        }
        self.dump = Dump::from(self.state.clone(), self.field.to_string());
        self.state = State::Input;
        self.field = Field::new();
    }
    pub async fn update(&mut self) {
        self.state = State::Game;
        self.question = Question::get_random();
        self.translation = self.question.trans().await;
        self.field = Field::new();
        self.result = 0;
    }
    pub async fn check(&mut self) {
        self.state = State::Check;
        let right = self.question.answer.clone();
        let left = self.field.to_string();
        if left.len() == right.len() {
            let mut counter = 0;
            for i in 0..right.len() {
                if left.chars().nth(i).unwrap() == right.chars().nth(i).unwrap() {
                    counter += 1;
                }
            }
            let persentage = counter * 100 / right.len() as u16;
            if persentage > 70 {
                self.score += 1;
            }
            self.translation = self.question.trans().await;
            self.result = persentage;
            if self.score >= consts::WIN_SCORE {
                self.state = State::Win;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dump {
    pub state: State,
    pub field: Field,
}
impl Dump {
    pub fn new() -> Self {
        Self {
            state: State::Input,
            field: Field::new(),
        }
    }
    pub fn from(state: State, text: String) -> Self {
        let field = Field::from(&text);
        Self { state, field }
    }
}
