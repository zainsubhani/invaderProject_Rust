use rusty_time::Timer;

struct Shot {
    x: i32,
    y: i32,
    pub explording: bool,
    timer: Timer,
}

impl Shot {
    fn new(x: i32, y: i32) -> Self {
        Shot {
            x,
            y,
            explording: true,
            timer: Timer::new(std::time::Duration::from_millis(100)),
        }
    }
    pub fn update(&mut self, delta: std::time::Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.explording {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }
    pub fn explording(&mut self) {
        self.explording = true;
        self.timer = Timer::new(std::time::Duration::from_millis(50));
    }
    pub fn dead(&self) -> bool {}
}
