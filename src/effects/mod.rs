pub mod quantum_core;
pub mod cyber_grid;
pub mod gravitational_nebula;

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
    effects: Vec<Box<dyn ShaderEffect>>,
    active_index: usize,
}

impl Default for EffectRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl EffectRegistry {
    pub fn new() -> Self {
        Self {
            effects: Vec::new(),
            active_index: 0,
        }
    }

    pub fn register(&mut self, effect: Box<dyn ShaderEffect>) {
        self.effects.push(effect);
    }

    pub fn effects_count(&self) -> usize {
        self.effects.len()
    }

    pub fn active_index(&self) -> usize {
        self.active_index
    }

    pub fn active_effect(&self) -> &dyn ShaderEffect {
        &(*self.effects[self.active_index])
    }

    pub fn active_effect_mut(&mut self) -> &mut dyn ShaderEffect {
        &mut (*self.effects[self.active_index])
    }

    pub fn switch_effect(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.effects.len() {
            return Err("Shader effect index out of bounds");
        }
        self.active_index = index;
        Ok(())
    }

    pub fn effect_names(&self) -> Vec<&'static str> {
        self.effects.iter().map(|e| e.name()).collect()
    }
}
