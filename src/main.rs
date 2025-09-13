use std::{error::Error, io, time::Duration};

use crossterm::{
    ExecutableCommand,
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let mut audio = Audio::new();
    audio.add("explode", "./src/sounds/explode.wav");
    audio.add("lose", "./src/sounds/lose.wav");
    audio.add("move", "./src/sounds/move.wav");
    audio.add("pew", "./src/sounds/pew.wav");
    audio.add("startup", "./src/sounds/startup.wav");
    audio.add("win", "./src/sounds/win.wav");
    audio.play("explode");

    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Game loop
    'gameloop: loop {
        // input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => (), // catch all other keys
                }
            }
        }
    }

    // cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
