use web_sys::{HtmlCanvasElement, WebGl2RenderingContext, WebGlProgram, WebGlShader};
use wasm_bindgen::JsValue;
use wasm_bindgen::JsCast;

use super::canvas::CanvasManager;
use super::input::InputController;
use super::uniforms::{EngineParams, UniformManager};
use super::buffer::GeometryBuffer;
use crate::effects::EffectRegistry;

pub struct ShaderEngine {
    gl: WebGl2RenderingContext,
    canvas_manager: CanvasManager,
    pub input: InputController,
    pub uniforms: UniformManager,
    geometry: GeometryBuffer,
    pub registry: EffectRegistry,
    pub params: EngineParams,
    current_program: Option<WebGlProgram>,
    last_frame_time: f64,
}

impl ShaderEngine {
    pub fn new(canvas: HtmlCanvasElement) -> Result<Self, JsValue> {
        let gl = canvas
            .get_context("webgl2")?
            .ok_or_else(|| JsValue::from_str("WebGL2 context not supported"))?
            .dyn_into::<WebGl2RenderingContext>()?;

        let canvas_manager = CanvasManager::new(canvas);
        canvas_manager.apply_viewport(&gl);

        let geometry = GeometryBuffer::new(&gl)?;
        let input = InputController::new();
        let uniforms = UniformManager::new();
        let registry = EffectRegistry::new();
        let params = EngineParams::default();

        let engine = Self {
            gl,
            canvas_manager,
            input,
            uniforms,
            geometry,
            registry,
            params,
            current_program: None,
            last_frame_time: 0.0,
        };

        Ok(engine)
    }

    pub fn compile_active_effect(&mut self) -> Result<(), JsValue> {
        let effect = self.registry.active_effect();
        let vs_source = effect.vertex_source();
        let fs_source = effect.fragment_source();

        let program = compile_shader_program(&self.gl, vs_source, fs_source)?;
        self.gl.use_program(Some(&program));

        // Cache common uniforms
        let uniform_names = [
            "u_time",
            "u_resolution",
            "u_mouse",
            "u_velocity",
            "u_scroll",
            "u_params",
            "u_ripples",
        ];
        self.uniforms.cache_locations(&self.gl, &program, &uniform_names);

        // Allow active effect to initialize any custom resources
        self.registry.active_effect_mut().init(&self.gl, &program)?;

        // Set default params for active effect
        let (speed, warp, glow, color_shift) = self.registry.active_effect().default_params();
        self.params.set_speed(speed);
        self.params.set_warp(warp);
        self.params.set_glow(glow);
        self.params.set_color_shift(color_shift);

        self.current_program = Some(program);
        Ok(())
    }

    pub fn switch_effect(&mut self, index: usize) -> Result<(), JsValue> {
        self.registry
            .switch_effect(index)
            .map_err(|e| JsValue::from_str(e))?;
        self.compile_active_effect()?;
        Ok(())
    }

    pub fn set_mobile_mode(&mut self, is_mobile: bool) -> Result<(), JsValue> {
        // DPR cap differs per mode, so the backing store may resize; keep the viewport in sync
        if self.canvas_manager.set_mobile(is_mobile) {
            self.canvas_manager.apply_viewport(&self.gl);
        }
        self.registry.set_mobile_mode(is_mobile);
        self.compile_active_effect()?;
        Ok(())
    }

    pub fn resize(&mut self) {
        if self.canvas_manager.resize_to_window() {
            self.canvas_manager.apply_viewport(&self.gl);
        }
    }

    pub fn render(&mut self, current_time_ms: f64) -> Result<(), JsValue> {
        self.resize();

        let dt = if self.last_frame_time > 0.0 {
            ((current_time_ms - self.last_frame_time) / 1000.0).min(0.1) as f32
        } else {
            0.016
        };
        self.last_frame_time = current_time_ms;

        // Update controllers
        self.input.update(dt);
        self.registry.active_effect_mut().update(dt, &self.input);

        // Upload uniforms
        let time_sec = (current_time_ms / 1000.0) as f32;
        self.uniforms.set_1f(&self.gl, "u_time", time_sec);
        self.uniforms.set_2f(
            &self.gl,
            "u_resolution",
            self.canvas_manager.width(),
            self.canvas_manager.height(),
        );

        let (ndc_x, ndc_y) = self.input.normalized_coords(
            self.canvas_manager.width(),
            self.canvas_manager.height(),
        );
        self.uniforms.set_2f(&self.gl, "u_mouse", ndc_x, ndc_y);

        let (vx, vy) = self.input.velocity();
        self.uniforms.set_2f(&self.gl, "u_velocity", vx, vy);
        self.uniforms.set_1f(&self.gl, "u_scroll", self.input.scroll());

        self.uniforms.set_4f(
            &self.gl,
            "u_params",
            self.params.speed,
            self.params.warp,
            self.params.glow,
            self.params.color_shift,
        );

        let ripples = self.input.ripples_data();
        self.uniforms.set_4fv(&self.gl, "u_ripples", &ripples);

        // Bind any custom effect uniforms
        self.registry.active_effect().bind_custom_uniforms(&self.gl, &self.uniforms, &self.params)?;

        // Draw quad
        self.geometry.bind(&self.gl);
        self.geometry.draw(&self.gl);

        Ok(())
    }
}

fn compile_shader_program(
    gl: &WebGl2RenderingContext,
    vs_source: &str,
    fs_source: &str,
) -> Result<WebGlProgram, JsValue> {
    let vs = compile_shader(gl, WebGl2RenderingContext::VERTEX_SHADER, vs_source)?;
    let fs = compile_shader(gl, WebGl2RenderingContext::FRAGMENT_SHADER, fs_source)?;

    let program = gl
        .create_program()
        .ok_or_else(|| JsValue::from_str("Unable to create shader program"))?;

    gl.attach_shader(&program, &vs);
    gl.attach_shader(&program, &fs);
    gl.link_program(&program);

    if !gl
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let info = gl
            .get_program_info_log(&program)
            .unwrap_or_else(|| "Unknown program link error".to_string());
        return Err(JsValue::from_str(&format!("Shader link error: {}", info)));
    }

    Ok(program)
}

fn compile_shader(
    gl: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, JsValue> {
    let shader = gl
        .create_shader(shader_type)
        .ok_or_else(|| JsValue::from_str("Unable to create shader"))?;

    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if !gl
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        let info = gl
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| "Unknown shader compile error".to_string());
        return Err(JsValue::from_str(&format!("Shader compile error: {}", info)));
    }

    Ok(shader)
}
