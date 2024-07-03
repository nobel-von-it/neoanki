use std::rc::Rc;

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{consts, main, states::State};

use super::{
    game::Game,
    heplers::{centered_rect, get_centered},
};

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
        .split(get_main_layouts(f)[0])
}
fn get_footer_layouts(f: &mut Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(get_main_layouts(f)[1])
}
fn get_game_layouts(f: &mut Frame) -> Rc<[Rect]> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            // Pseudo padding
            Constraint::Percentage(20),
            // Question
            Constraint::Length(3),
            // Question translation
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(get_body_layouts(f)[1])
}

impl Game {
    pub fn draw_ui(&self, f: &mut Frame) {
        let game_layout = get_game_layouts(f);
        let footer_layout = get_footer_layouts(f);
        let score = get_centered(format!("Score: {}", self.score));
        let state = get_centered(self.state.to_string());
        match self.state {
            State::Input => {
                let field = Paragraph::new(Text::from(self.field.show())).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(consts::COMMAND_TEXT),
                );

                f.render_widget(field, centered_rect(40, 10, f.size()));
            }
            State::Game => {
                let question = get_centered(self.question.sentence.clone())
                    .block(Block::default().borders(Borders::ALL));
                let field = get_centered(self.field.show());

                f.render_widget(question, game_layout[1]);
                f.render_widget(field, game_layout[6]);
                f.render_widget(score, footer_layout[0]);
                f.render_widget(state, footer_layout[1]);
            }
            State::Check => {
                let main_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Percentage(100), Constraint::Length(1)])
                    .split(f.size());
                let game_layout = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Percentage(50),
                        Constraint::Length(3),
                        Constraint::Length(1),
                        Constraint::Length(3),
                        Constraint::Length(3),
                        Constraint::Length(1),
                        Constraint::Percentage(50),
                    ])
                    .split(
                        Layout::default()
                            .direction(Direction::Horizontal)
                            .constraints([
                                Constraint::Percentage(20),
                                Constraint::Percentage(60),
                                Constraint::Percentage(20),
                            ])
                            .split(main_layout[0])[1],
                    );
                let footer_layout = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                    .split(main_layout[1]);

                let translation = get_centered(self.translation.clone());

                let question = Paragraph::new(Text::from(self.question.sentence.clone()))
                    .alignment(Alignment::Center)
                    .block(Block::default().borders(Borders::ALL));

                let field = Paragraph::new(Text::from(format!(
                    "Ожидалось: {}, введено: {}. Верно на {}%",
                    self.field, self.question.answer, self.result
                )))
                .alignment(Alignment::Center)
                .block(Block::default().borders(Borders::ALL));

                f.render_widget(question, game_layout[1]);
                f.render_widget(translation, game_layout[2]);
                f.render_widget(field, game_layout[4]);

                f.render_widget(score, footer_layout[0]);
                f.render_widget(state, footer_layout[1]);
            }
            State::QuestionManager => {}
            State::Win => {}
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
}
