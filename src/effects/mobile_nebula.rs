use super::ShaderEffect;

pub struct NebulaDriftMobileEffect;

impl Default for NebulaDriftMobileEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl NebulaDriftMobileEffect {
    pub fn new() -> Self {
        Self
    }
}

impl ShaderEffect for NebulaDriftMobileEffect {
    fn name(&self) -> &'static str {
        "Nebula Drift (Mobile)"
    }

    fn description(&self) -> &'static str {
        "Lightweight domain-warped nebula with drifting star dust and a touch-reactive gravity vortex"
    }

    fn default_params(&self) -> (f32, f32, f32, f32) {
        (0.8, 1.2, 1.2, 0.3)
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

mat2 rot(float a) {
    float c = cos(a), s = sin(a);
    return mat2(c, -s, s, c);
}

float hash(vec2 p) {
    p = fract(p * vec2(123.34, 456.21));
    p += dot(p, p + 45.32);
    return fract(p.x * p.y);
}

float noise(vec2 p) {
    vec2 i = floor(p);
    vec2 f = fract(p);
    vec2 u = f * f * (3.0 - 2.0 * f);

    float a = hash(i);
    float b = hash(i + vec2(1.0, 0.0));
    float c = hash(i + vec2(0.0, 1.0));
    float d = hash(i + vec2(1.0, 1.0));

    return mix(mix(a, b, u.x), mix(c, d, u.x), u.y);
}

// 4 octaves (desktop nebula uses 5) keeps the per-pixel noise budget mobile friendly
float fbm(vec2 p) {
    float v = 0.0;
    float a = 0.5;
    mat2 m = rot(0.4);
    for (int i = 0; i < 4; i++) {
        v += a * noise(p);
        p = m * p * 2.02;
        a *= 0.5;
    }
    return v;
}

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);

    // Scroll advances the flow phase rather than shifting uv; the noise field fills every pixel
    float t = u_time * u_params.x * 0.4 + u_scroll * 0.001;
    float warp = u_params.y;
    float glow_mult = u_params.z;
    float color_shift = u_params.w;

    // Touch gravity vortex
    vec2 touch = u_mouse * 0.5;
    vec2 to_touch = uv - touch;
    float touch_dist = length(to_touch);
    float vortex = exp(-touch_dist * 3.0) * 0.8;
    uv += rot(vortex * 4.0) * to_touch * vortex * 0.5;

    // Ripple shockwaves
    for (int i = 0; i < 5; i++) {
        if (u_ripples[i].w > 0.001) {
            vec2 r_pos = u_ripples[i].xy * 0.5;
            float d = length(uv - r_pos);
            float ring = sin(d * 20.0 - u_ripples[i].z * 8.0) * exp(-d * 3.0);
            uv += normalize(uv - r_pos + 1e-4) * ring * 0.03 * u_ripples[i].w;
        }
    }

    // Single-level domain warp (desktop uses two): 3 fbm calls instead of 5
    vec2 q = vec2(
        fbm(uv * 1.6 + t * 0.2),
        fbm(uv * 1.6 + vec2(5.2, 1.3) - t * 0.15)
    );
    float f = fbm(uv * 1.4 + 3.5 * q * warp + vec2(1.7, 9.2) + t * 0.1);

    vec3 col_deep = vec3(0.02, 0.03, 0.08);
    vec3 col_teal = vec3(0.0, 0.55, 0.8);
    vec3 col_violet = vec3(0.45, 0.15, 0.85);
    vec3 col_rose = vec3(0.95, 0.35, 0.6);

    col_teal = mix(col_teal, vec3(0.0, 0.9, 0.6), sin(color_shift * 6.28) * 0.5 + 0.5);

    vec3 color = mix(col_deep, col_teal, clamp(f * f * 3.5, 0.0, 1.0));
    color = mix(color, col_violet, clamp(length(q) * 0.9, 0.0, 1.0));
    color = mix(color, col_rose, clamp(q.y * q.y * 1.5, 0.0, 1.0) * 0.5);

    color *= (f * 1.8 + 0.3) * glow_mult;

    // Touch point-light flare
    color += vec3(0.3, 0.7, 1.0) * exp(-touch_dist * 4.0) * 0.6 * glow_mult;

    // Twinkling star dust: a soft dot inside sparse cells, rather than filling the whole cell
    vec2 star_grid = uv * 70.0;
    float star = hash(floor(star_grid));
    if (star > 0.985) {
        float dot_mask = smoothstep(0.3, 0.0, length(fract(star_grid) - 0.5));
        color += vec3(0.8, 0.9, 1.0) * dot_mask * (sin(t * 4.0 + star * 100.0) * 0.5 + 0.5) * 0.8;
    }

    float vig = 1.0 - length(v_uv - 0.5) * 0.8;
    color *= clamp(vig, 0.25, 1.0);

    fragColor = vec4(color, 1.0);
}
"#
    }
}
