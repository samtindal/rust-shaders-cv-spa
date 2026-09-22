use super::ShaderEffect;

pub struct QuantumPulsarMobileEffect;

impl Default for QuantumPulsarMobileEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl QuantumPulsarMobileEffect {
    pub fn new() -> Self {
        Self
    }
}

impl ShaderEffect for QuantumPulsarMobileEffect {
    fn name(&self) -> &'static str {
        "Quantum Pulsar (Mobile)"
    }

    fn description(&self) -> &'static str {
        "Analytical 2D quantum pulsar with harmonic ripples and gyroscopic touch reaction"
    }

    fn default_params(&self) -> (f32, f32, f32, f32) {
        (1.0, 1.2, 1.3, 0.1)
    }

    fn fragment_source(&self) -> &'static str {
        r#"#version 300 es
precision highp float;

in vec2 v_uv;
out vec4 fragColor;

uniform float u_time;
uniform vec2 u_resolution;
uniform vec2 u_mouse;
uniform vec2 u_velocity;
uniform float u_scroll;
uniform vec4 u_params; // x: speed, y: warp, z: glow, w: color_shift
uniform vec4 u_ripples[5];

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);

    // Scroll advances the animation phase rather than shifting uv, so the pulsar stays centered
    float t = u_time * u_params.x * 0.8 + u_scroll * 0.0015;
    float warp = u_params.y;
    float glow_mult = u_params.z;
    float color_shift = u_params.w;

    // Mobile touch attractor
    vec2 touch = u_mouse * 0.4;
    vec2 p = uv - touch;

    float dist = length(p);

    // Dynamic pulsating radius
    float pulse = sin(t * 3.0) * 0.04 + sin(t * 6.0) * 0.02;
    float r = 0.35 + pulse;

    // Outer harmonic wave interference rings
    float ring1 = abs(dist - 0.55 - sin(t * 1.5) * 0.05);
    float ring2 = abs(dist - 0.75 - cos(t * 2.0) * 0.04 * warp);
    float ring3 = abs(dist - 0.95 - sin(t * 2.5) * 0.03 * warp);

    float glow_core = 0.04 / (abs(dist - r) + 0.06);
    float glow_r1 = 0.015 / (ring1 + 0.04);
    float glow_r2 = 0.012 / (ring2 + 0.04);
    float glow_r3 = 0.009 / (ring3 + 0.04);

    vec3 cyan = vec3(0.05, 0.5, 1.0);
    vec3 magenta = vec3(0.9, 0.15, 0.8);
    vec3 gold = vec3(1.0, 0.8, 0.2);

    vec3 core_col = mix(cyan, magenta, sin(t * 0.5 + color_shift * 6.28) * 0.5 + 0.5);
    vec3 ring_col = mix(magenta, gold, cos(t * 0.7) * 0.5 + 0.5);

    vec3 col = vec3(0.015, 0.02, 0.06);
    col += core_col * glow_core * 1.8 * glow_mult;
    col += ring_col * (glow_r1 + glow_r2 + glow_r3) * 1.2 * glow_mult;

    // Ripple shockwaves
    for (int i = 0; i < 5; i++) {
        if (u_ripples[i].w > 0.001) {
            vec2 r_pos = u_ripples[i].xy * 0.5;
            float d = length(uv - r_pos);
            float wave = sin(d * 24.0 - u_ripples[i].z * 10.0) * exp(-d * 3.5);
            col += cyan * wave * 0.25 * u_ripples[i].w;
        }
    }

    // Vignette
    float vig = 1.0 - length(v_uv - 0.5) * 0.8;
    col *= clamp(vig, 0.25, 1.0);

    fragColor = vec4(col, 1.0);
}
"#
    }
}
