const PROGRESS_BAR_WIDTH: u32 = 40;

pub struct LoadingBar {
    max: f32,
    step: f32,
    progress: f32,
}

impl LoadingBar {
    pub fn new(max: f32, step: f32) -> LoadingBar {
        LoadingBar {
            max: max,
            step: step,
            progress: 0.0,
        }
    }

    pub fn reset(&mut self) {
        self.progress = 0.0;
    }

    pub fn step(&mut self) {
        self.progress += self.step;
    }

    pub fn loading_bar_string(&self) -> String {
        let mut output = String::from(format!("Progress: ({}%) [", (200.0 * (self.progress/self.max)) as i32));
        for i in 0..PROGRESS_BAR_WIDTH {
            let rp = i as f32 / PROGRESS_BAR_WIDTH as f32;
            let np = self.progress / self.max / 0.5;
            if rp <= np {
                output.push('=');
            } else {
                output.push(' ');
            }
        }

        output.push(']');
        output
    }
}