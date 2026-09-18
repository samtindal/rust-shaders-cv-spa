use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlUniformLocation};
use std::collections::HashMap;

#[derive(Clone, Copy, Debug)]
pub struct EngineParams {
    pub speed: f32,
    pub warp: f32,
    pub glow: f32,
    pub color_shift: f32,
}

impl Default for EngineParams {
    fn default() -> Self {
        Self {
            speed: 1.0,
            warp: 1.0,
            glow: 1.0,
            color_shift: 0.0,
        }
    }
}

impl EngineParams {
    pub fn new(speed: f32, warp: f32, glow: f32, color_shift: f32) -> Self {
        let mut params = Self::default();
        params.set_speed(speed);
        params.set_warp(warp);
        params.set_glow(glow);
        params.set_color_shift(color_shift);
        params
    }

    pub fn set_speed(&mut self, val: f32) {
        self.speed = val.clamp(0.1, 5.0);
    }

    pub fn set_warp(&mut self, val: f32) {
        self.warp = val.clamp(0.0, 3.0);
    }

    pub fn set_glow(&mut self, val: f32) {
        self.glow = val.clamp(0.0, 3.0);
    }

    pub fn set_color_shift(&mut self, val: f32) {
        self.color_shift = val.clamp(0.0, 1.0);
    }
}

pub struct UniformManager {
    locations: HashMap<String, WebGlUniformLocation>,
}

impl Default for UniformManager {
    fn default() -> Self {
        Self::new()
    }
}

impl UniformManager {
    pub fn new() -> Self {
        Self {
            locations: HashMap::new(),
        }
    }

    pub fn cache_locations(&mut self, gl: &WebGl2RenderingContext, program: &WebGlProgram, uniform_names: &[&str]) {
        self.locations.clear();
        for &name in uniform_names {
            if let Some(loc) = gl.get_uniform_location(program, name) {
                self.locations.insert(name.to_string(), loc);
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<&WebGlUniformLocation> {
        self.locations.get(name)
    }

    pub fn set_1f(&self, gl: &WebGl2RenderingContext, name: &str, v: f32) {
        if let Some(loc) = self.get(name) {
            gl.uniform1f(Some(loc), v);
        }
    }

    pub fn set_2f(&self, gl: &WebGl2RenderingContext, name: &str, x: f32, y: f32) {
        if let Some(loc) = self.get(name) {
            gl.uniform2f(Some(loc), x, y);
        }
    }

    pub fn set_4f(&self, gl: &WebGl2RenderingContext, name: &str, x: f32, y: f32, z: f32, w: f32) {
        if let Some(loc) = self.get(name) {
            gl.uniform4f(Some(loc), x, y, z, w);
        }
    }

    pub fn set_4fv(&self, gl: &WebGl2RenderingContext, name: &str, values: &[f32]) {
        if let Some(loc) = self.get(name) {
            gl.uniform4fv_with_f32_array(Some(loc), values);
        }
    }
}
