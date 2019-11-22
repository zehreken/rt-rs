pub mod fps_utils {
    pub struct FpsCounter {
        time_as_second: u128,
        frames: i32,
    }

    impl FpsCounter {
        pub fn new() -> FpsCounter {
            FpsCounter {
                time_as_second: 0,
                frames: 0,
            }
        }

        pub fn tick(&mut self, delta_ms: u128) -> i32 {
            self.time_as_second += delta_ms;
            self.frames += 1;
            if self.time_as_second >= 1000 {
                self.time_as_second -= 1000;
                let temp = self.frames;
                self.frames = 0;
                return temp;
            }

            return 0;
        }
    }
}
