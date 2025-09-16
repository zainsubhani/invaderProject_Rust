use std::{
    error::Error,
    io::{self},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    ExecutableCommand,
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use invader::{
    frame::{self, Drawable, new_frame},
    player::Player,
    render::{self},
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

    // Render loop in a seprate thread
    let (render_tx, render_rx) = mpsc::channel();
    let render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game loop
    let mut player = Player::new();
    let mut instant = Instant::now();
    'gameloop: loop {
        // per frame init
        let delta = instant.elapsed();
        instant = Instant::now();

        let mut curr_frame = new_frame();

        // input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Left => player.move_left(),
                    KeyCode::Right => player.move_right(),
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Esc | KeyCode::Char('q') | KeyCode::Char('Q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => (), // catch all other keys
                }
            }
        }
        // Updates
        player.update(delta);

        // Draw and Render Section
        player.draw(&mut curr_frame);

        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // cleanup
    drop(render_tx);
    render_handle.join().unwrap();

    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
