use sam_shader_engine::engine::input::{InputController, RippleEvent};
use sam_shader_engine::engine::uniforms::EngineParams;
use sam_shader_engine::effects::{EffectRegistry, ShaderEffect};
use sam_shader_engine::effects::quantum_core::QuantumCoreEffect;
use sam_shader_engine::effects::cyber_grid::CyberGridEffect;
use sam_shader_engine::effects::gravitational_nebula::GravitationalNebulaEffect;

#[test]
fn test_input_controller_pointer_smoothing() {
    let mut controller = InputController::new();
    assert_eq!(controller.mouse_x(), 0.0);
    assert_eq!(controller.mouse_y(), 0.0);

    controller.on_pointer_move(100.0, 200.0);
    assert_eq!(controller.mouse_x(), 100.0);
    assert_eq!(controller.mouse_y(), 200.0);

    // Initial smooth coordinates will move towards target over time
    controller.update(0.016);
    assert!(controller.smooth_x() > 0.0 && controller.smooth_x() <= 100.0);
    assert!(controller.smooth_y() > 0.0 && controller.smooth_y() <= 200.0);
}

#[test]
fn test_input_controller_normalized_coordinates() {
    let mut controller = InputController::new();
    controller.on_pointer_move(400.0, 300.0);

    // With canvas 800x600:
    // Center (400, 300) should normalize to around (0.0, 0.0) in WebGL coordinate space [-1, 1]
    let (ndc_x, ndc_y) = controller.normalized_coords(800.0, 600.0);
    assert!((ndc_x - 0.0).abs() < 0.001);
    assert!((ndc_y - 0.0).abs() < 0.001);

    // Top-left (0, 0)
    controller.on_pointer_move(0.0, 0.0);
    let (tl_x, tl_y) = controller.normalized_coords(800.0, 600.0);
    assert!((tl_x - (-1.0)).abs() < 0.001);
    assert!((tl_y - 1.0).abs() < 0.001); // Inverted Y for WebGL
}

#[test]
fn test_ripple_shockwaves_creation_and_decay() {
    let mut controller = InputController::new();
    assert_eq!(controller.active_ripples_count(), 0);

    controller.on_pointer_down(0.5, -0.2);
    assert_eq!(controller.active_ripples_count(), 1);

    // Add multiple ripples up to max limit (e.g. 5)
    for i in 0..10 {
        controller.on_pointer_down(0.1 * i as f32, 0.1 * i as f32);
    }
    assert!(controller.active_ripples_count() <= 5, "Should cap maximum concurrent ripples");

    // Advance time past ripple lifespan (e.g. 2.0 seconds)
    controller.update(2.5);
    assert_eq!(controller.active_ripples_count(), 0, "Decayed ripples should be evicted");
}

#[test]
fn test_engine_params_clamping_and_defaults() {
    let mut params = EngineParams::default();
    assert_eq!(params.speed, 1.0);
    assert_eq!(params.warp, 1.0);
    assert_eq!(params.glow, 1.0);

    // Clamping checks
    params.set_speed(999.0);
    assert!(params.speed <= 5.0);

    params.set_speed(-10.0);
    assert!(params.speed >= 0.1);

    params.set_warp(-5.0);
    assert!(params.warp >= 0.0);

    params.set_glow(100.0);
    assert!(params.glow <= 3.0);
}

#[test]
fn test_effect_registry_polymorphic_dispatch() {
    let mut registry = EffectRegistry::new();
    assert_eq!(registry.effects_count(), 0);

    registry.register(Box::new(QuantumCoreEffect::new()));
    registry.register(Box::new(CyberGridEffect::new()));
    registry.register(Box::new(GravitationalNebulaEffect::new()));

    assert_eq!(registry.effects_count(), 3);
    assert_eq!(registry.active_effect().name(), "Quantum Core");

    // Switch effect
    let switch_result = registry.switch_effect(1);
    assert!(switch_result.is_ok());
    assert_eq!(registry.active_effect().name(), "Cyber Grid");

    let switch_result2 = registry.switch_effect(2);
    assert!(switch_result2.is_ok());
    assert_eq!(registry.active_effect().name(), "Gravitational Nebula");

    // Out of bounds check
    let invalid_switch = registry.switch_effect(99);
    assert!(invalid_switch.is_err());
    // Should remain on current
    assert_eq!(registry.active_effect().name(), "Gravitational Nebula");
}

#[test]
fn test_effects_have_valid_shaders() {
    let effects: Vec<Box<dyn ShaderEffect>> = vec![
        Box::new(QuantumCoreEffect::new()),
        Box::new(CyberGridEffect::new()),
        Box::new(GravitationalNebulaEffect::new()),
    ];

    for effect in effects {
        let vs = effect.vertex_source();
        let fs = effect.fragment_source();

        assert!(vs.contains("#version 300 es"), "Vertex shader must be GLSL 300 es");
        assert!(fs.contains("#version 300 es"), "Fragment shader must be GLSL 300 es");
        assert!(fs.contains("out vec4 fragColor"), "Fragment shader must declare output");
        assert!(fs.contains("u_time"), "Fragment shader must accept u_time uniform");
        assert!(fs.contains("u_resolution"), "Fragment shader must accept u_resolution uniform");
    }
}
