pub mod quantum_core;
pub mod cyber_grid;
pub mod gravitational_nebula;
pub mod mobile_nebula;
pub mod mobile_filaments;
pub mod mobile_pulsar;

use web_sys::{WebGl2RenderingContext, WebGlProgram};
use wasm_bindgen::JsValue;
use crate::engine::uniforms::{EngineParams, UniformManager};
use crate::engine::input::InputController;

/// The common vertex shader for full-screen quad rendering in WebGL2 (GLSL 300 es)
pub const COMMON_VERTEX_SHADER: &str = r#"#version 300 es
in vec2 a_position;
out vec2 v_uv;

void main() {
    v_uv = (a_position + 1.0) * 0.5;
    gl_Position = vec4(a_position, 0.0, 1.0);
}
"#;

pub trait ShaderEffect {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn vertex_source(&self) -> &'static str {
        COMMON_VERTEX_SHADER
    }
    fn fragment_source(&self) -> &'static str;
    fn default_params(&self) -> (f32, f32, f32, f32);

    fn init(&mut self, _gl: &WebGl2RenderingContext, _program: &WebGlProgram) -> Result<(), JsValue> {
        Ok(())
    }

    fn update(&mut self, _dt: f32, _input: &InputController) {}

    fn bind_custom_uniforms(
        &self,
        _gl: &WebGl2RenderingContext,
        _uniforms: &UniformManager,
        _params: &EngineParams,
    ) -> Result<(), JsValue> {
        Ok(())
    }
}

pub struct EffectRegistry {
    desktop_effects: Vec<Box<dyn ShaderEffect>>,
    mobile_effects: Vec<Box<dyn ShaderEffect>>,
    is_mobile: bool,
    desktop_index: usize,
    mobile_index: usize,
}

impl Default for EffectRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl EffectRegistry {
    pub fn new() -> Self {
        Self {
            desktop_effects: Vec::new(),
            mobile_effects: Vec::new(),
            is_mobile: false,
            desktop_index: 0,
            mobile_index: 0,
        }
    }

    pub fn register(&mut self, effect: Box<dyn ShaderEffect>) {
        self.desktop_effects.push(effect);
    }

    pub fn register_mobile(&mut self, effect: Box<dyn ShaderEffect>) {
        self.mobile_effects.push(effect);
    }

    pub fn is_mobile(&self) -> bool {
        self.is_mobile
    }

    pub fn set_mobile_mode(&mut self, is_mobile: bool) {
        self.is_mobile = is_mobile;
    }

    fn current_effects(&self) -> &Vec<Box<dyn ShaderEffect>> {
        if self.is_mobile && !self.mobile_effects.is_empty() {
            &self.mobile_effects
        } else {
            &self.desktop_effects
        }
    }

    fn current_effects_mut(&mut self) -> &mut Vec<Box<dyn ShaderEffect>> {
        if self.is_mobile && !self.mobile_effects.is_empty() {
            &mut self.mobile_effects
        } else {
            &mut self.desktop_effects
        }
    }

    pub fn effects_count(&self) -> usize {
        self.current_effects().len()
    }

    pub fn active_index(&self) -> usize {
        if self.is_mobile && !self.mobile_effects.is_empty() {
            self.mobile_index
        } else {
            self.desktop_index
        }
    }

    pub fn active_effect(&self) -> &dyn ShaderEffect {
        let idx = self.active_index();
        &(*self.current_effects()[idx])
    }

    pub fn active_effect_mut(&mut self) -> &mut dyn ShaderEffect {
        let idx = self.active_index();
        let effects = self.current_effects_mut();
        &mut (*effects[idx])
    }

    pub fn switch_effect(&mut self, index: usize) -> Result<(), &'static str> {
        let count = self.current_effects().len();
        if index >= count {
            return Err("Shader effect index out of bounds");
        }
        if self.is_mobile && !self.mobile_effects.is_empty() {
            self.mobile_index = index;
        } else {
            self.desktop_index = index;
        }
        Ok(())
    }

    pub fn effect_names(&self) -> Vec<&'static str> {
        self.current_effects().iter().map(|e| e.name()).collect()
    }
}
