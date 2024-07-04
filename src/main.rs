mod consts;
mod data;
mod flags;
mod game;
mod input;
mod states;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::{error, info};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};
use std::io::stdout;
use std::{fs::File, io::Write};

use game::game::Game;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    enable_raw_mode()?;
    execute!(
        stdout(),
        EnterAlternateScreen,
        EnableMouseCapture,
        crossterm::cursor::Hide,
        crossterm::terminal::SetTitle("NeoAnki"),
    )?;

    info!("NeoAnki started");

    let mut term = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut game = Game::start();
    let res = run(&mut term, &mut game).await;

    disable_raw_mode()?;
    execute!(
        stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        crossterm::cursor::Show
    )?;

    match res {
        Ok(_) => info!("NeoAnki stopped"),
        Err(e) => {
            error!("NeoAnki stopped with error: {}", e);
            std::process::exit(1)
        }
    };
    let mut info = std::fs::read_to_string("log/info.log").unwrap_or_else(|e| {
        error!("Failed to read log file: {}", e);
        let _ = std::fs::File::create("log/info.log");
        String::new()
    });
    info.push('\n');

    let mut file = File::create("log/info.log")?;
    file.write_all(info.as_bytes())?;
    Ok(())
}

async fn run<B: Backend>(term: &mut Terminal<B>, game: &mut Game) -> anyhow::Result<()> {
    info!("Game loop started");
    loop {
        term.draw(|f| game.draw_ui(f))?;
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            match game.state {
                states::State::Input => match key.code {
                    KeyCode::Esc => game.switch(),
                    KeyCode::Left => game.field.left(),
                    KeyCode::Right => game.field.right(),
                    KeyCode::Backspace => game.field.del(),
                    KeyCode::Char(c) => game.field.add(c),
                    KeyCode::Enter => match game.field.get_command() {
                        states::InputCommands::Start => {
                            game.update().await;
                        }
                        states::InputCommands::Exit => {
                            break;
                        }
                        states::InputCommands::Add
                        | states::InputCommands::Remove
                        | states::InputCommands::Edit => {
                            game.state = states::State::QuestionManager;
                        }
                        states::InputCommands::Help => {}
                        _ => {}
                    },
                    _ => {}
                },
                states::State::Game => match key.code {
                    KeyCode::Esc => game.switch(),
                    KeyCode::Left => game.field.left(),
                    KeyCode::Right => game.field.right(),
                    KeyCode::Backspace => game.field.del(),
                    KeyCode::Char(c) => game.field.add(c),
                    KeyCode::Enter => {
                        game.check().await;
                    }
                    _ => {}
                },
                states::State::Check => game.update().await,
                states::State::Win => break,
                _ => {}
            }
        }
    }
    Ok(())
}

// use clap::{Parser, Subcommand};
//
// #[derive(Debug, Parser)]
// pub struct Args {
//     #[clap(subcommand)]
//     pub command: Option<Command>,
// }
//
// #[derive(Debug, Subcommand)]
// pub enum Command {
//     #[clap(arg_required_else_help = true)]
//     Add {
//         #[clap(short, long)]
//         something: String,
//     },
// }
