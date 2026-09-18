use super::ShaderEffect;

pub struct GravitationalNebulaEffect;

impl Default for GravitationalNebulaEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl GravitationalNebulaEffect {
    pub fn new() -> Self {
        Self
    }
}

impl ShaderEffect for GravitationalNebulaEffect {
    fn name(&self) -> &'static str {
        "Gravitational Nebula"
    }

    fn description(&self) -> &'static str {
        "Fluid fractal Brownian motion deep space nebula with gravitational mouse vortex and chromatic dispersion"
    }

    fn default_params(&self) -> (f32, f32, f32, f32) {
        (0.8, 1.3, 1.2, 0.4)
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

// 2D Rotation
mat2 rot(float a) {
    float c = cos(a), s = sin(a);
    return mat2(c, -s, s, c);
}

// Pseudo-random and noise
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

// Multi-octave fractal Brownian motion with domain warping
float fbm(vec2 p) {
    float v = 0.0;
    float a = 0.5;
    mat2 m = rot(0.4);
    for (int i = 0; i < 5; i++) {
        v += a * noise(p);
        p = m * p * 2.02;
        a *= 0.5;
    }
    return v;
}

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);
    uv.y += u_scroll * 0.0002;

    float t = u_time * u_params.x * 0.4;
    float warp = u_params.y;
    float glow_mult = u_params.z;
    float color_shift = u_params.w;

    // Gravitational mouse attractor & vortex
    vec2 mouse_ndc = u_mouse * 0.5;
    vec2 to_mouse = uv - mouse_ndc;
    float mouse_dist = length(to_mouse);
    float vortex_strength = exp(-mouse_dist * 3.0) * 0.8;
    uv += rot(vortex_strength * 4.0) * to_mouse * vortex_strength * 0.5;

    // Ripples shockwave
    for (int i = 0; i < 5; i++) {
        if (u_ripples[i].w > 0.001) {
            vec2 r_pos = u_ripples[i].xy * 0.5;
            float d = length(uv - r_pos);
            float ring = sin(d * 20.0 - u_ripples[i].z * 8.0) * exp(-d * 3.0);
            uv += normalize(uv - r_pos + 1e-4) * ring * 0.03 * u_ripples[i].w;
        }
    }

    // Domain Warping via fBm
    vec2 q = vec2(
        fbm(uv * 1.8 + vec2(0.0, 0.0) + t * 0.2),
        fbm(uv * 1.8 + vec2(5.2, 1.3) + t * 0.25)
    );

    vec2 r = vec2(
        fbm(uv * 2.2 + 4.0 * q * warp + vec2(1.7, 9.2) + t * 0.3),
        fbm(uv * 2.2 + 4.0 * q * warp + vec2(8.3, 2.8) + t * 0.35)
    );

    float f = fbm(uv * 1.5 + 4.0 * r * warp);

    // Deep space chromatic nebula palettes
    vec3 col_deep = vec3(0.02, 0.03, 0.08); // Dark cosmic void
    vec3 col_plasma = vec3(0.1, 0.5, 0.9);   // Electric blue plasma
    vec3 col_core = vec3(0.9, 0.2, 0.6);     // Magenta stellar core
    vec3 col_flare = vec3(0.98, 0.85, 0.3);  // Gold ion flare

    // Shift colors based on uniform
    col_plasma = mix(col_plasma, vec3(0.0, 0.9, 0.6), sin(color_shift * 6.28) * 0.5 + 0.5);

    vec3 color = mix(col_deep, col_plasma, clamp((f * f) * 3.5, 0.0, 1.0));
    color = mix(color, col_core, clamp(length(q), 0.0, 1.0));
    color = mix(color, col_flare, clamp(length(r.x), 0.0, 1.0) * 0.6);

    // Multiply by glow parameter
    color *= (f * 1.8 + 0.3) * glow_mult;

    // Mouse point-light flare
    color += vec3(0.3, 0.7, 1.0) * exp(-mouse_dist * 4.0) * 0.8 * glow_mult;

    // Star dust particles
    float star_noise = hash(floor(uv * 80.0));
    if (star_noise > 0.97) {
        color += vec3(0.8, 0.9, 1.0) * (sin(t * 4.0 + star_noise * 100.0) * 0.5 + 0.5) * 0.6;
    }

    // Vignette
    float vig = 1.0 - length(v_uv - 0.5) * 0.8;
    color *= clamp(vig, 0.25, 1.0);

    fragColor = vec4(color, 1.0);
}
"#
    }
}
