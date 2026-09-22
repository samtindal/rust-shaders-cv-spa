#[derive(Clone, Copy, Debug)]
pub struct RippleEvent {
    pub x: f32,
    pub y: f32,
    pub age: f32,
    pub max_life: f32,
    pub intensity: f32,
}

impl RippleEvent {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            age: 0.0,
            max_life: 2.0,
            intensity: 1.0,
        }
    }

    pub fn is_alive(&self) -> bool {
        self.age < self.max_life
    }
}

pub struct InputController {
    mouse_x: f32,
    mouse_y: f32,
    smooth_x: f32,
    smooth_y: f32,
    velocity_x: f32,
    velocity_y: f32,
    scroll_y: f32,
    smooth_scroll: f32,
    ripples: Vec<RippleEvent>,
    max_ripples: usize,
}

impl Default for InputController {
    fn default() -> Self {
        Self::new()
    }
}

impl InputController {
    pub fn new() -> Self {
        Self {
            mouse_x: 0.0,
            mouse_y: 0.0,
            smooth_x: 0.0,
            smooth_y: 0.0,
            velocity_x: 0.0,
            velocity_y: 0.0,
            scroll_y: 0.0,
            smooth_scroll: 0.0,
            ripples: Vec::new(),
            max_ripples: 5,
        }
    }

    pub fn mouse_x(&self) -> f32 {
        self.mouse_x
    }

    pub fn mouse_y(&self) -> f32 {
        self.mouse_y
    }

    pub fn smooth_x(&self) -> f32 {
        self.smooth_x
    }

    pub fn smooth_y(&self) -> f32 {
        self.smooth_y
    }

    pub fn velocity(&self) -> (f32, f32) {
        (self.velocity_x, self.velocity_y)
    }

    pub fn scroll(&self) -> f32 {
        self.smooth_scroll
    }

    pub fn active_ripples_count(&self) -> usize {
        self.ripples.len()
    }

    pub fn on_pointer_move(&mut self, x: f32, y: f32) {
        self.mouse_x = x;
        self.mouse_y = y;
    }

    pub fn on_pointer_down(&mut self, x: f32, y: f32) {
        if self.ripples.len() >= self.max_ripples {
            self.ripples.remove(0);
        }
        self.ripples.push(RippleEvent::new(x, y));
    }

    /// `scroll_y` is the absolute page offset (window.scrollY), not a delta
    pub fn on_scroll(&mut self, scroll_y: f32) {
        self.scroll_y = scroll_y;
    }

    pub fn update(&mut self, dt: f32) {
        let lerp_factor = (dt * 6.0).min(1.0);
        let prev_smooth_x = self.smooth_x;
        let prev_smooth_y = self.smooth_y;

        self.smooth_x += (self.mouse_x - self.smooth_x) * lerp_factor;
        self.smooth_y += (self.mouse_y - self.smooth_y) * lerp_factor;

        if dt > 0.0 {
            self.velocity_x = (self.smooth_x - prev_smooth_x) / dt;
            self.velocity_y = (self.smooth_y - prev_smooth_y) / dt;
        }

        self.smooth_scroll += (self.scroll_y - self.smooth_scroll) * (dt * 4.0).min(1.0);

        // Update and filter ripples
        for ripple in &mut self.ripples {
            ripple.age += dt;
            ripple.intensity = (1.0 - ripple.age / ripple.max_life).max(0.0);
        }
        self.ripples.retain(|r| r.is_alive());
    }

    /// Converts canvas pixel coordinates to WebGL normalized device coordinates [-1.0, 1.0]
    pub fn normalized_coords(&self, width: f32, height: f32) -> (f32, f32) {
        if width <= 0.0 || height <= 0.0 {
            return (0.0, 0.0);
        }
        let ndc_x = (self.mouse_x / width) * 2.0 - 1.0;
        let ndc_y = 1.0 - (self.mouse_y / height) * 2.0;
        (ndc_x, ndc_y)
    }

    /// Flattens ripples into an array for WebGL uniform upload: [x, y, age, intensity, ...]
    pub fn ripples_data(&self) -> [f32; 20] {
        let mut data = [0.0f32; 20];
        for (i, r) in self.ripples.iter().take(5).enumerate() {
            data[i * 4] = r.x;
            data[i * 4 + 1] = r.y;
            data[i * 4 + 2] = r.age;
            data[i * 4 + 3] = r.intensity;
        }
        data
    }
}
