use super::ShaderEffect;

pub struct QuantumCoreEffect;

impl Default for QuantumCoreEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl QuantumCoreEffect {
    pub fn new() -> Self {
        Self
    }
}

impl ShaderEffect for QuantumCoreEffect {
    fn name(&self) -> &'static str {
        "Quantum Core"
    }

    fn description(&self) -> &'static str {
        "Raymarched 4D quantum core with chromatic aberration, orbiting rings, and gyroscopic mouse reaction"
    }

    fn default_params(&self) -> (f32, f32, f32, f32) {
        // (speed, warp, glow, color_shift)
        (1.0, 1.2, 1.4, 0.15)
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
uniform vec4 u_ripples[5]; // x, y: center ndc, z: age, w: intensity

// Rotation matrix
mat2 rot(float a) {
    float c = cos(a), s = sin(a);
    return mat2(c, -s, s, c);
}

// Signed distance functions
float sdSphere(vec3 p, float r) {
    return length(p) - r;
}

float sdTorus(vec3 p, vec2 t) {
    vec2 q = vec2(length(p.xz) - t.x, p.y);
    return length(q) - t.y;
}

float sdBox(vec3 p, vec3 b) {
    vec3 d = abs(p) - b;
    return min(max(d.x, max(d.y, d.z)), 0.0) + length(max(d, 0.0));
}

// Scene distance estimation
float map(vec3 p, float t, float warp) {
    // Gyroscopic rotation influenced by mouse
    vec3 q = p;
    q.yz *= rot(u_mouse.y * 0.8 + t * 0.2);
    q.xz *= rot(u_mouse.x * 0.8 + t * 0.3);

    // Inner pulsating core
    float pulse = sin(t * 2.5) * 0.08 + sin(t * 5.0) * 0.03;
    float core = sdSphere(q, 0.65 + pulse);

    // Outer gyro-toruses
    vec3 t1_p = q;
    t1_p.xy *= rot(t * 0.7);
    float t1 = sdTorus(t1_p, vec2(1.15, 0.035 * warp));

    vec3 t2_p = q;
    t2_p.yz *= rot(t * 0.9 + 1.0);
    float t2 = sdTorus(t2_p, vec2(1.35, 0.025 * warp));

    vec3 t3_p = q;
    t3_p.xz *= rot(t * 1.1 + 2.0);
    float t3 = sdTorus(t3_p, vec2(1.55, 0.02 * warp));

    // Morphing shell lattice
    vec3 shell_p = q;
    shell_p = abs(shell_p) - 0.75;
    float shell = sdBox(shell_p, vec3(0.08 * warp));

    float rings = min(min(t1, t2), t3);
    return min(core, min(rings, shell));
}

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution) / min(u_resolution.x, u_resolution.y);
    
    // Parallax scroll reaction
    uv.y += u_scroll * 0.0003;

    // Ripple shockwave distortion
    for(int i = 0; i < 5; i++) {
        if(u_ripples[i].w > 0.001) {
            vec2 r_center = u_ripples[i].xy * 0.5;
            float dist = length(uv - r_center);
            float wave_rad = u_ripples[i].z * 0.8;
            float ripple = sin((dist - wave_rad) * 35.0) * exp(-abs(dist - wave_rad) * 12.0);
            uv += normalize(uv - r_center + 1e-4) * ripple * 0.04 * u_ripples[i].w;
        }
    }

    float t = u_time * u_params.x * 0.6;
    float warp = u_params.y;
    float glow_mult = u_params.z;
    float color_shift = u_params.w;

    // Camera setup
    vec3 ro = vec3(0.0, 0.0, -3.4);
    vec3 rd = normalize(vec3(uv, 1.2));

    // Raymarching loop with volumetric glow accumulation
    float d = 0.0;
    float glow = 0.0;
    float ring_glow = 0.0;

    for (int i = 0; i < 64; i++) {
        vec3 p = ro + rd * d;
        float dist = map(p, t, warp);
        
        // Volumetric glow accumulation
        glow += 0.015 / (abs(dist) + 0.04);
        
        if (dist < 0.001 || d > 8.0) break;
        d += dist * 0.75;
    }

    // Color gradient composition
    vec3 colA = vec3(0.05, 0.45, 0.95); // Deep electric cyan-blue
    vec3 colB = vec3(0.75, 0.15, 0.95); // Royal magenta
    vec3 colC = vec3(0.10, 0.95, 0.70); // Emerald teal

    // Shift colors based on shift param and mouse
    vec3 core_color = mix(colA, colB, sin(t * 0.5 + color_shift * 6.28) * 0.5 + 0.5);
    core_color = mix(core_color, colC, (u_mouse.x * 0.5 + 0.5) * 0.4);

    vec3 final_color = glow * core_color * 0.07 * glow_mult;

    // Starfield / subtle dust particles
    vec2 p_uv = uv * 3.0;
    float stars = fract(sin(dot(p_uv, vec2(12.9898, 78.233))) * 43758.5453);
    if (stars > 0.985) {
        final_color += vec3(0.4, 0.6, 0.9) * (stars - 0.985) * 40.0 * (sin(t * 3.0 + stars * 10.0) * 0.5 + 0.5);
    }

    // Subtle dark vignette to make foreground resume text pop effortlessly
    float vignette = 1.0 - length(v_uv - 0.5) * 0.85;
    final_color *= clamp(vignette, 0.3, 1.0);

    // Deep background tint
    final_color += vec3(0.02, 0.03, 0.06);

    fragColor = vec4(final_color, 1.0);
}
"#
    }
}
