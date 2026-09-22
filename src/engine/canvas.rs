use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};

pub struct CanvasManager {
    canvas: HtmlCanvasElement,
    width: u32,
    height: u32,
    dpr: f64,
    is_mobile: bool,
}

impl CanvasManager {
    pub fn new(canvas: HtmlCanvasElement) -> Self {
        let mut manager = Self {
            canvas,
            width: 0,
            height: 0,
            dpr: 1.0,
            is_mobile: false,
        };
        manager.resize_to_window();
        manager
    }

    pub fn set_mobile(&mut self, is_mobile: bool) -> bool {
        self.is_mobile = is_mobile;
        self.resize_to_window()
    }

    pub fn resize_to_window(&mut self) -> bool {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return false,
        };

        let max_dpr = if self.is_mobile { 1.25 } else { 2.0 };
        self.dpr = window.device_pixel_ratio().min(max_dpr);
        let client_w = window
            .inner_width()
            .ok()
            .and_then(|w| w.as_f64())
            .unwrap_or(800.0) as u32;
        let client_h = window
            .inner_height()
            .ok()
            .and_then(|h| h.as_f64())
            .unwrap_or(600.0) as u32;

        let display_w = (client_w as f64 * self.dpr) as u32;
        let display_h = (client_h as f64 * self.dpr) as u32;

        if self.width != display_w || self.height != display_h {
            self.width = display_w;
            self.height = display_h;
            self.canvas.set_width(display_w);
            self.canvas.set_height(display_h);
            return true;
        }
        false
    }

    pub fn apply_viewport(&self, gl: &WebGl2RenderingContext) {
        gl.viewport(0, 0, self.width as i32, self.height as i32);
    }

    pub fn width(&self) -> f32 {
        self.width as f32
    }

    pub fn height(&self) -> f32 {
        self.height as f32
    }

    pub fn dpr(&self) -> f64 {
        self.dpr
    }
}
