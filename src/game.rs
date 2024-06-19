use crate::{
    data::Question,
    states::{InputCommands, State},
};

#[derive(Debug, Clone)]
pub struct Game {
    pub state: State,
    pub question: Question,
    pub field: Field,
    pub result: String,
    pub index: usize,
}
impl Game {
    pub fn start() -> Self {
        let state = State::Input;
        let question = Question::get_random();
        let field = Field::new();
        let result = String::new();
        let index = 0;
        Self {
            state,
            question,
            field,
            result,
            index,
        }
    }
    pub fn cmp(&self) -> Option<String> {
        let right = self.question.answer.clone();
        let left = self.field.to_string();
        if left.len() == right.len() {
            // string with + or - symbols
            let mut res = String::new();
            for i in 0..left.len() {
                if left.chars().nth(i).unwrap() == right.chars().nth(i).unwrap() {
                    res.push('+');
                } else {
                    res.push('-');
                }
            }
            return Some(res);
        }
        // None means full incorrect answer
        None
    }
    pub fn draw(&self) {
        println!("\x1B[2J\x1B[1;1H");
        println!("{}: {}", self.field.pos, self.field.show());
        println!("{}", self.result);
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
        if self.pos < self.text.len() - 1 && self.text.len() != 0 {
            self.pos += 1
        }
    }
    pub fn add(&mut self, c: char) {
        self.text.insert(self.pos + 1, c);
        self.right();
    }
    pub fn del(&mut self) {
        if self.pos < self.text.len() {
            self.text.remove(self.pos);
            self.left();
        }
    }
    pub fn to_string(&self) -> String {
        self.text.iter().collect()
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
                    format!("{}", c)
                } else {
                    format!("_")
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }
}
