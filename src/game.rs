use crate::consts;
use core::fmt;
use std::{io::stdout, rc::Rc};

use crossterm::execute;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::{block::BlockExt, Block, Borders, Paragraph},
    Frame,
};

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
    pub score: u16,
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
    pub async fn check(&mut self) {
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
            self.result = format!("{} --- {}", res, persentage);
            if self.score >= consts::WIN_SCORE {
                self.state = State::Win;
            }
        }
    }
    // pub async fn draw(&self) -> anyhow::Result<()> {
    //     // clear terminal and move cursor to top
    //     execute!(
    //         stdout(),
    //         crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
    //         crossterm::cursor::MoveTo(0, 0)
    //     )?;
    //     let (x, y) = crossterm::terminal::size()?;
    //     match self.state {
    //         State::Input => {
    //             execute!(
    //                 stdout(),
    //                 crossterm::cursor::MoveTo((x / 2) - (x / 4), consts::QUESTION_POS)
    //             )?;
    //             println!("{}", consts::COMMAND_TEXT);
    //
    //             self.draw_nubers(x, y)?;
    //
    //             execute!(
    //                 stdout(),
    //                 crossterm::cursor::MoveTo((x / 2) - (x / 4), consts::QUESTION_POS + 1)
    //             )?;
    //             println!("{}", self.field.show());
    //         }
    //         State::Game => {
    //             execute!(
    //                 stdout(),
    //                 crossterm::cursor::MoveTo(
    //                     (x / 2) - (self.question.sentence.len() as u16 / 2),
    //                     consts::QUESTION_POS
    //                 )
    //             )?;
    //             println!("{}", self.question.sentence);
    //
    //             // execute!(
    //             //     stdout(),
    //             //     crossterm::cursor::MoveTo((x / 2) - (self.translation.len() as u16 / 2), 3)
    //             // )?;
    //             // print!("{}", self.translation.trim());
    //
    //             self.draw_footer(x, y)?;
    //             self.draw_nubers(x, y)?;
    //
    //             execute!(
    //                 stdout(),
    //                 crossterm::cursor::MoveTo(
    //                     (x / 2) - (self.field.text.len() as u16 / 2),
    //                     consts::QUESTION_POS + 3
    //                 ),
    //             )?;
    //             println!("{}", self.field.show());
    //         }
    //         State::Check => {
    //             execute!(
    //                 stdout(),
    //                 crossterm::cursor::MoveTo(
    //                     (x / 2) - (self.question.sentence.len() as u16 / 2),
    //                     consts::QUESTION_POS
    //                 )
    //             )?;
    //             println!("{}", self.question.sentence);
    //
    //             execute!(
    //                 stdout(),
    //                 crossterm::cursor::MoveTo(
    //                     (x / 2) - (self.translation.len() as u16 / 2),
    //                     consts::QUESTION_POS + 1
    //                 )
    //             )?;
    //             println!("{}", self.translation);
    //
    //             execute!(
    //                 stdout(),
    //                 crossterm::cursor::MoveTo(
    //                     (x / 2) - (self.question.answer.len() as u16 / 2),
    //                     consts::QUESTION_POS + 3
    //                 )
    //             )?;
    //             println!("{}", self.question.answer);
    //
    //             execute!(
    //                 stdout(),
    //                 crossterm::cursor::MoveTo(
    //                     (x / 2) - (self.field.to_string().len() as u16 / 2),
    //                     consts::QUESTION_POS + 4
    //                 )
    //             )?;
    //             println!("{}", self.field);
    //
    //             execute!(
    //                 stdout(),
    //                 crossterm::cursor::MoveTo(
    //                     (x / 2) - (self.field.to_string().len() as u16 / 2),
    //                     consts::QUESTION_POS + 5
    //                 )
    //             )?;
    //             println!("{}", self.result);
    //
    //             self.draw_footer(x, y)?;
    //         }
    //         State::QuestionManager => {}
    //         State::Win => {}
    //     }
    //
    //     Ok(())
    // }
    pub fn draw_ui(&self, f: &mut Frame) {
        let game_layout = Self::get_game_layouts(f);
        let footer_layout = Self::get_footer_layouts(f);
        match self.state {
            State::Input => {
                let field = Self::get_centered(self.field.show()).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(consts::COMMAND_TEXT),
                );

                f.render_widget(field, Self::centered_rect(40, 10, f.size()));
            }
            State::Game => {
                let question = Self::get_centered(self.question.sentence.clone())
                    .block(Block::default().borders(Borders::ALL));
                let field = Self::get_centered(self.field.show())
                    .block(Block::default().borders(Borders::BOTTOM));

                let (score, state) = self.get_score_and_state();

                f.render_widget(question, game_layout[1]);
                f.render_widget(field, game_layout[6]);
                f.render_widget(score, footer_layout[0]);
                f.render_widget(state, footer_layout[1]);
            }
            State::Check => {
                let question = Paragraph::new(Text::raw(&self.question.sentence).centered());
                let translation = Paragraph::new(Text::raw(&self.translation).centered());
                let answer = Paragraph::new(Text::raw(&self.question.answer).centered());
                let field = Paragraph::new(Text::raw(self.field.show()).centered());
                let result = Paragraph::new(Text::raw(&self.result).centered());

                let (score, state) = self.get_score_and_state();

                f.render_widget(question, game_layout[1]);
                f.render_widget(translation, game_layout[2]);
                f.render_widget(answer, game_layout[5]);
                f.render_widget(field, game_layout[6]);
                f.render_widget(result, game_layout[7]);

                f.render_widget(score, footer_layout[0]);
                f.render_widget(state, footer_layout[1]);
            }
            State::QuestionManager => {}
            State::Win => {}
        }
    }
    fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
        // Cut the given rectangle into three vertical pieces
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(r);

        // Then cut the middle vertical piece into three width-wise pieces
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1] // Return the middle chunk
    }
    fn get_centered<'a>(text: String) -> Paragraph<'a> {
        Paragraph::new(Text::raw(text).centered())
    }
    fn get_score_and_state(&self) -> (Paragraph, Paragraph) {
        let score = Paragraph::new(format!("Score: {}", self.score));
        let state = Paragraph::new(format!("State: {}", self.state));
        (score, state)
    }
    fn get_main_layouts(f: &mut Frame) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(100), Constraint::Length(1)])
            .split(f.size())
    }
    fn get_body_layouts(f: &mut Frame) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Percentage(60),
                Constraint::Percentage(20),
            ])
            .split(Self::get_main_layouts(f)[0])
    }
    fn get_footer_layouts(f: &mut Frame) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(Self::get_main_layouts(f)[1])
    }
    fn get_game_layouts(f: &mut Frame) -> Rc<[Rect]> {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Length(3),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(Self::get_body_layouts(f)[1])
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
