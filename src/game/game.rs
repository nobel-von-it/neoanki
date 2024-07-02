use crate::{consts, data::Question, states::State};

use super::field::Field;

#[derive(Debug, Clone)]
pub struct Game {
    pub state: State,
    pub last_state: GameState,
    pub question: Question,
    pub translation: String,
    pub field: Field,
    pub result: String,
    pub score: u16,
}
impl Game {
    pub fn start() -> Self {
        let state = State::Input;
        let last_state = GameState::new();
        let question = Question::default();
        let translation = String::new();
        let field = Field::new();
        let result = String::new();
        Self {
            state,
            last_state,
            question,
            translation,
            field,
            result,
            score: 0,
        }
    }
    pub fn switch(&mut self) {
        // if self.state == State::Input && self.last_state == State::Input {
        //     return;
        // }
        // if self.state == State::Input {
        //     self.state = self.last_state.clone();
        //     return;
        // }
        // self.last_state = self.state.clone();
        // self.state = State::Input;
        if self.state == State::Input && self.last_state.state == State::Input {
            return;
        }
        if self.state == State::Input {
            self.state = self.last_state.state.clone();
            self.field = Field::from(&self.last_state.field.to_string());
            return;
        }
        self.last_state = GameState::from(self.state.clone(), self.field.to_string());
        self.state = State::Input;
        self.field = Field::new();
    }
    pub async fn update(&mut self) {
        self.state = State::Game;
        self.question = Question::get_random();
        self.translation = self.question.trans().await;
        self.field = Field::new();
        self.result = String::new();
    }
    pub async fn check(&mut self) {
        self.state = State::Check;
        let right = self.question.answer.clone();
        let left = self.field.to_string();
        if left.len() == right.len() {
            let mut res = String::new();
            let mut counter = 0;
            for i in 0..right.len() {
                if left.chars().nth(i).unwrap() == right.chars().nth(i).unwrap() {
                    res.push('+');
                    counter += 1;
                } else {
                    res.push('-');
                }
            }
            let persentage = counter * 100 / right.len() as u16;
            if persentage > 70 {
                self.score += 1;
            }
            self.translation = self.question.trans().await;
            self.result = format!("{} == {}%", res, persentage);
            if self.score >= consts::WIN_SCORE {
                self.state = State::Win;
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GameState {
    pub state: State,
    pub field: Field,
}
impl GameState {
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
