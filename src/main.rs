mod consts;
mod data;
mod flags;
mod game;
mod states;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use game::Game;
use std::io::stdout;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    let mut game = Game::start();
    let res = run(&mut game);

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    res?;
    Ok(())
}

fn run(game: &mut Game) -> anyhow::Result<()> {
    loop {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Release {
                continue;
            }
            execute!(
                stdout(),
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
                crossterm::cursor::MoveTo(0, 0)
            )?;
            match key.code {
                KeyCode::Esc => game.state = states::State::Input,
                KeyCode::Left => game.field.left(),
                KeyCode::Right => game.field.right(),
                KeyCode::Backspace => game.field.del(),
                KeyCode::Char(c) => game.field.add(c),
                KeyCode::Enter => match game.state {
                    states::State::Input => match game.field.get_command() {
                        states::InputCommands::Start => {
                            game.state = states::State::Game;
                        }
                        states::InputCommands::Exit => {
                            break;
                        }
                        _ => {}
                    },
                    states::State::Game => {
                        game.state = states::State::Input;
                    }
                    _ => {}
                },
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
