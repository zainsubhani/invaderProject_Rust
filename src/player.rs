use crate::{NUM_COLS, NUM_ROWS, frame::Drawable};
use std::time::Duration;

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<crate::shot::Shot>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: NUM_COLS / 2,
            y: NUM_ROWS - 1,
            shots: Vec::new(),
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < NUM_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots
                .push(crate::shot::Shot::new(self.x as i32, self.y as i32 - 1));
            true
        } else {
            false
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = "A"; // fixed: char not &str
        for shot in self.shots.iter() {
            // fixed: use field, not method
            shot.draw(frame);
        }
    }
}
