use std::{error::Error, io};

use crossterm::{
    ExecutableCommand,
    cursor::{Hide, Show},
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

    audio.play("lose");

    // terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // cleanup
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
