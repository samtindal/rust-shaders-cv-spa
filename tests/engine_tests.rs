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

#[test]
fn test_mobile_effects_and_mode_switching() {
    use sam_shader_engine::effects::mobile_nebula::NebulaDriftMobileEffect;
    use sam_shader_engine::effects::mobile_filaments::IonFilamentsMobileEffect;
    use sam_shader_engine::effects::mobile_pulsar::QuantumPulsarMobileEffect;

    let mut registry = EffectRegistry::new();

    // Register desktop effects
    registry.register(Box::new(QuantumCoreEffect::new()));
    registry.register(Box::new(CyberGridEffect::new()));
    registry.register(Box::new(GravitationalNebulaEffect::new()));

    // Register mobile effects
    registry.register_mobile(Box::new(NebulaDriftMobileEffect::new()));
    registry.register_mobile(Box::new(IonFilamentsMobileEffect::new()));
    registry.register_mobile(Box::new(QuantumPulsarMobileEffect::new()));

    assert!(!registry.is_mobile(), "Default should be desktop mode");
    assert_eq!(registry.effects_count(), 3);
    assert_eq!(registry.active_effect().name(), "Quantum Core");

    // Switch to mobile mode
    registry.set_mobile_mode(true);
    assert!(registry.is_mobile());
    assert_eq!(registry.effects_count(), 3);
    assert_eq!(registry.active_effect().name(), "Nebula Drift (Mobile)");

    let names = registry.effect_names();
    assert_eq!(names, vec!["Nebula Drift (Mobile)", "Ion Filaments (Mobile)", "Quantum Pulsar (Mobile)"]);

    // Test mobile effect preset switching
    assert!(registry.switch_effect(1).is_ok());
    assert_eq!(registry.active_effect().name(), "Ion Filaments (Mobile)");

    assert!(registry.switch_effect(2).is_ok());
    assert_eq!(registry.active_effect().name(), "Quantum Pulsar (Mobile)");

    // Switch back to desktop mode
    registry.set_mobile_mode(false);
    assert!(!registry.is_mobile());
    assert_eq!(registry.active_effect().name(), "Quantum Core");
}

#[test]
fn test_mobile_shaders_are_valid_and_non_raymarched() {
    use sam_shader_engine::effects::mobile_nebula::NebulaDriftMobileEffect;
    use sam_shader_engine::effects::mobile_filaments::IonFilamentsMobileEffect;
    use sam_shader_engine::effects::mobile_pulsar::QuantumPulsarMobileEffect;

    let mobile_effects: Vec<Box<dyn ShaderEffect>> = vec![
        Box::new(NebulaDriftMobileEffect::new()),
        Box::new(IonFilamentsMobileEffect::new()),
        Box::new(QuantumPulsarMobileEffect::new()),
    ];

    for effect in mobile_effects {
        let vs = effect.vertex_source();
        let fs = effect.fragment_source();

        assert!(vs.contains("#version 300 es"));
        assert!(fs.contains("#version 300 es"));
        assert!(fs.contains("out vec4 fragColor"));
        assert!(fs.contains("u_time"));
        assert!(fs.contains("u_resolution"));
        assert!(fs.contains("u_mouse"));
        // Mobile shaders should NOT contain heavy raymarch loops (for < 64 etc.)
        assert!(!fs.contains("for (int i = 0; i < 64; i++)"), "Mobile shaders must not use heavy 64-step raymarching loops");
    }
}

#[test]
fn test_scroll_tracks_absolute_position_not_accumulated_events() {
    // JS forwards window.scrollY (absolute) on every scroll event; a touch swipe fires many
    let mut controller = InputController::new();
    for _ in 0..50 {
        controller.on_scroll(1000.0);
    }
    for _ in 0..200 {
        controller.update(0.016);
    }
    assert!((controller.scroll() - 1000.0).abs() < 1.0, "scroll drifted to {}", controller.scroll());

    // Scrolling back to the top must return the parallax offset to zero
    controller.on_scroll(0.0);
    for _ in 0..200 {
        controller.update(0.016);
    }
    assert!(controller.scroll().abs() < 1.0, "scroll stuck at {}", controller.scroll());
}

#[test]
fn test_mobile_shaders_keep_content_on_screen_when_scrolled() {
    use sam_shader_engine::effects::mobile_nebula::NebulaDriftMobileEffect;
    use sam_shader_engine::effects::mobile_filaments::IonFilamentsMobileEffect;
    use sam_shader_engine::effects::mobile_pulsar::QuantumPulsarMobileEffect;

    // The single-column phone layout scrolls ~8500px; a linear scroll offset on uv or the
    // horizon slides the whole effect off-canvas. Scroll must drive phase, not position.
    let mobile_effects: Vec<Box<dyn ShaderEffect>> = vec![
        Box::new(NebulaDriftMobileEffect::new()),
        Box::new(IonFilamentsMobileEffect::new()),
        Box::new(QuantumPulsarMobileEffect::new()),
    ];

    for effect in mobile_effects {
        let fs = effect.fragment_source();
        assert!(fs.contains("u_scroll"), "{} should still react to scroll", effect.name());
        assert!(!fs.contains("uv.y += u_scroll"), "{} offsets uv by scroll", effect.name());
        assert!(!fs.contains("- u_scroll *"), "{} offsets the horizon by scroll", effect.name());
    }
}
