pub mod engine;
pub mod effects;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{HtmlCanvasElement, window};

use crate::engine::ShaderEngine;
use crate::effects::quantum_core::QuantumCoreEffect;
use crate::effects::cyber_grid::CyberGridEffect;
use crate::effects::gravitational_nebula::GravitationalNebulaEffect;
use crate::effects::mobile_nebula::NebulaDriftMobileEffect;
use crate::effects::mobile_filaments::IonFilamentsMobileEffect;
use crate::effects::mobile_pulsar::QuantumPulsarMobileEffect;

#[wasm_bindgen]
pub struct ResumeShaderApp {
    engine: ShaderEngine,
}

#[wasm_bindgen]
impl ResumeShaderApp {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<ResumeShaderApp, JsValue> {
        #[cfg(feature = "console_error_panic_hook")]
        console_error_panic_hook::set_once();

        let window = window().ok_or_else(|| JsValue::from_str("Window not available"))?;
        let document = window.document().ok_or_else(|| JsValue::from_str("Document not available"))?;
        let element = document
            .get_element_by_id(canvas_id)
            .ok_or_else(|| JsValue::from_str(&format!("Canvas with id '{}' not found", canvas_id)))?;

        let canvas = element
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| JsValue::from_str("Element is not an HtmlCanvasElement"))?;

        let mut engine = ShaderEngine::new(canvas)?;

        // Register default desktop shader effects
        engine.registry.register(Box::new(QuantumCoreEffect::new()));
        engine.registry.register(Box::new(CyberGridEffect::new()));
        engine.registry.register(Box::new(GravitationalNebulaEffect::new()));

        // Register battery-friendly mobile-optimized shader effects
        engine.registry.register_mobile(Box::new(NebulaDriftMobileEffect::new()));
        engine.registry.register_mobile(Box::new(IonFilamentsMobileEffect::new()));
        engine.registry.register_mobile(Box::new(QuantumPulsarMobileEffect::new()));

        // Compile initial active effect
        engine.compile_active_effect()?;

        Ok(ResumeShaderApp { engine })
    }

    #[wasm_bindgen]
    pub fn set_mobile_mode(&mut self, is_mobile: bool) -> Result<(), JsValue> {
        self.engine.set_mobile_mode(is_mobile)
    }

    #[wasm_bindgen]
    pub fn is_mobile_mode(&self) -> bool {
        self.engine.registry.is_mobile()
    }

    #[wasm_bindgen]
    pub fn switch_effect(&mut self, index: usize) -> Result<(), JsValue> {
        self.engine.switch_effect(index)
    }

    #[wasm_bindgen]
    pub fn get_effects_list(&self) -> js_sys::Array {
        let array = js_sys::Array::new();
        for name in self.engine.registry.effect_names() {
            array.push(&JsValue::from_str(name));
        }
        array
    }

    #[wasm_bindgen]
    pub fn get_active_index(&self) -> usize {
        self.engine.registry.active_index()
    }

    #[wasm_bindgen]
    pub fn get_active_name(&self) -> String {
        self.engine.registry.active_effect().name().to_string()
    }

    #[wasm_bindgen]
    pub fn get_active_description(&self) -> String {
        self.engine.registry.active_effect().description().to_string()
    }

    #[wasm_bindgen]
    pub fn set_speed(&mut self, val: f32) {
        self.engine.params.set_speed(val);
    }

    #[wasm_bindgen]
    pub fn set_warp(&mut self, val: f32) {
        self.engine.params.set_warp(val);
    }

    #[wasm_bindgen]
    pub fn set_glow(&mut self, val: f32) {
        self.engine.params.set_glow(val);
    }

    #[wasm_bindgen]
    pub fn set_color_shift(&mut self, val: f32) {
        self.engine.params.set_color_shift(val);
    }

    #[wasm_bindgen]
    pub fn get_params(&self) -> js_sys::Float32Array {
        let params = [
            self.engine.params.speed,
            self.engine.params.warp,
            self.engine.params.glow,
            self.engine.params.color_shift,
        ];
        unsafe { js_sys::Float32Array::view(&params) }
    }

    #[wasm_bindgen]
    pub fn on_pointer_move(&mut self, x: f32, y: f32) {
        self.engine.input.on_pointer_move(x, y);
    }

    #[wasm_bindgen]
    pub fn on_pointer_down(&mut self, x: f32, y: f32) {
        self.engine.input.on_pointer_down(x, y);
    }

    #[wasm_bindgen]
    pub fn on_scroll(&mut self, scroll_y: f32) {
        self.engine.input.on_scroll(scroll_y);
    }

    #[wasm_bindgen]
    pub fn resize(&mut self) {
        self.engine.resize();
    }

    #[wasm_bindgen]
    pub fn render(&mut self, time_ms: f64) -> Result<(), JsValue> {
        self.engine.render(time_ms)
    }
}
