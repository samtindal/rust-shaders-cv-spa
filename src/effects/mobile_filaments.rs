use super::ShaderEffect;

pub struct IonFilamentsMobileEffect;

impl Default for IonFilamentsMobileEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl IonFilamentsMobileEffect {
    pub fn new() -> Self {
        Self
    }
}

impl ShaderEffect for IonFilamentsMobileEffect {
    fn name(&self) -> &'static str {
        "Ion Filaments (Mobile)"
    }

    fn description(&self) -> &'static str {
        "Ridged-noise plasma filaments threading through ionized haze, bending around your touch"
    }

    fn default_params(&self) -> (f32, f32, f32, f32) {
        (0.7, 1.2, 1.2, 0.2)
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

// Ridged fbm: folding noise around its midpoint turns smooth clouds into thin bright strands
float ridged(vec2 p) {
    float v = 0.0;
    float a = 0.5;
    mat2 m = rot(0.7);
    for (int i = 0; i < 4; i++) {
        float n = 1.0 - abs(noise(p) * 2.0 - 1.0);
        v += a * n * n;
        p = m * p * 2.03;
        a *= 0.5;
    }
    return v;
}

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);

    // Scroll advances the flow phase rather than shifting uv; the noise field fills every pixel
    float t = u_time * u_params.x * 0.35 + u_scroll * 0.001;
    float warp = u_params.y;
    float glow_mult = u_params.z;
    float color_shift = u_params.w;

    // Filaments bend around the touch point
    vec2 touch = u_mouse * 0.5;
    vec2 to_touch = uv - touch;
    float touch_dist = length(to_touch);
    float bend = exp(-touch_dist * 3.0) * 0.8;
    uv += rot(bend * 3.0) * to_touch * bend * 0.4;

    // Ripple shockwaves
    for (int i = 0; i < 5; i++) {
        if (u_ripples[i].w > 0.001) {
            vec2 r_pos = u_ripples[i].xy * 0.5;
            float d = length(uv - r_pos);
            float ring = sin(d * 20.0 - u_ripples[i].z * 8.0) * exp(-d * 3.0);
            uv += normalize(uv - r_pos + 1e-4) * ring * 0.03 * u_ripples[i].w;
        }
    }

    // Soft haze field, which also warps the filaments so they flow with it
    vec2 q = vec2(
        fbm(uv * 1.2 + t * 0.15),
        fbm(uv * 1.2 + vec2(3.1, 7.4) - t * 0.12)
    );
    float fil = ridged(uv * 1.8 + 2.5 * q * warp + vec2(t * 0.05, -t * 0.04));

    vec3 col_deep = vec3(0.02, 0.02, 0.07);
    vec3 col_haze = vec3(0.25, 0.1, 0.55);
    vec3 col_arc = vec3(0.95, 0.2, 0.7);
    vec3 col_hot = vec3(1.0, 0.8, 0.35);

    col_arc = mix(col_arc, vec3(0.1, 0.6, 1.0), sin(color_shift * 6.28) * 0.5 + 0.5);

    vec3 color = col_deep + col_haze * q.x * q.x * 1.6;
    float strand = pow(fil, 2.5);
    color += mix(col_arc, col_hot, pow(fil, 4.0)) * strand * 1.6 * glow_mult;

    // Touch point-light flare
    color += vec3(0.9, 0.4, 1.0) * exp(-touch_dist * 4.0) * 0.5 * glow_mult;

    float vig = 1.0 - length(v_uv - 0.5) * 0.8;
    color *= clamp(vig, 0.25, 1.0);

    fragColor = vec4(color, 1.0);
}
"#
    }
}
