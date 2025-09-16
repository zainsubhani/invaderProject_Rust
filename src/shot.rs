use crate::frame::{Drawable, Frame};
use rusty_time::Timer;

pub struct Shot {
    x: i32,
    y: i32,
    pub exploding: bool,
    timer: Timer,
}

impl Shot {
    pub fn new(x: i32, y: i32) -> Self {
        Shot {
            x,
            y,
            exploding: false,
            timer: Timer::new(std::time::Duration::from_millis(100)),
        }
    }

    pub fn update(&mut self, delta: std::time::Duration) {
        self.timer.tick(delta); // <-- fixed
        if self.timer.finished() && !self.exploding {
            // <-- fixed
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }

    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::new(std::time::Duration::from_millis(50));
    }

    pub fn dead(&self) -> bool {
        (self.y == 0) || (self.exploding && self.timer.finished()) // <-- fixed
    }
}

impl Drawable for Shot {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x as usize][self.y as usize] = if self.exploding { "*" } else { "|" };
    }
}
