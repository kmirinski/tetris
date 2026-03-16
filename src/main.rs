use std::io::{self, stdout, Stdout};
use std::time::{Duration, Instant};
use crossterm::ExecutableCommand;
use crossterm::terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::{self, Event, KeyCode};
use ratatui::{Terminal, backend::CrosstermBackend};
use game::Game;

mod piece;
mod board;
mod game;
mod render;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let result = run(&mut terminal);

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    result
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    let mut game = Game::new(0);
    let mut last_tick = Instant::now();

    loop {
        terminal.draw(|f| render::render(f, &game))?;

        let gravity = Duration::from_millis(game.gravity_ms());
        let timeout = gravity.saturating_sub(last_tick.elapsed());

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Left                           => game.move_left(),
                    KeyCode::Right                          => game.move_right(),
                    KeyCode::Down                           => game.soft_drop(),
                    KeyCode::Char(' ')                      => game.hard_drop(),
                    KeyCode::Up                             => game.rotate(),
                    KeyCode::Char('r') if game.game_over    => game = Game::new(0),
                    KeyCode::Char('q') | KeyCode::Esc       => break,
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= gravity {
            game.tick();
            last_tick = Instant::now();
        }
    }
    Ok(())
}
