use super::ShaderEffect;

pub struct CyberGridEffect;

impl Default for CyberGridEffect {
    fn default() -> Self {
        Self::new()
    }
}

impl CyberGridEffect {
    pub fn new() -> Self {
        Self
    }
}

impl ShaderEffect for CyberGridEffect {
    fn name(&self) -> &'static str {
        "Cyber Grid"
    }

    fn description(&self) -> &'static str {
        "Infinite perspective synthwave cyber-grid with terrain elevation, horizon glow, and shockwave propagation"
    }

    fn default_params(&self) -> (f32, f32, f32, f32) {
        (1.0, 1.0, 1.3, 0.0)
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
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution) / u_resolution.y;

    float t = u_time * u_params.x * 0.8;
    float warp = u_params.y;
    float glow_mult = u_params.z;
    float color_shift = u_params.w;

    // Horizon line with slight mouse pitch/roll tilt
    float horizon = 0.08 + u_mouse.y * 0.12 - u_scroll * 0.00015;
    float tilt = u_mouse.x * 0.15;
    
    uv.x += (uv.y - horizon) * tilt;

    vec3 col = vec3(0.01, 0.02, 0.05); // Deep space background

    if (uv.y < horizon) {
        // Floor perspective projection
        float depth = 1.0 / (horizon - uv.y);
        vec2 p = vec2(uv.x * depth, depth + t * 2.0);

        // Terrain elevation harmonics
        float elevation = sin(p.x * 0.6) * cos(p.y * 0.4) * 0.6 * warp;
        elevation += sin(p.x * 1.5 + t) * 0.25;

        // Apply ripples in grid world space
        for(int i = 0; i < 5; i++) {
            if(u_ripples[i].w > 0.001) {
                vec2 r_pos = vec2(u_ripples[i].x * 3.0, depth - u_ripples[i].y * 2.0);
                float d = length(p - r_pos);
                elevation += sin(d * 4.0 - u_ripples[i].z * 10.0) * exp(-d * 0.2) * 0.8 * u_ripples[i].w;
            }
        }

        // Grid lines calculation
        vec2 grid = abs(fract(p - 0.5) - 0.5) / fwidth(p);
        float line = min(grid.x, grid.y);
        float grid_val = 1.0 - min(line, 1.0);

        // Depth fog / fading into horizon
        float fog = exp(-depth * 0.07);

        // Grid neon color palette
        vec3 neon_cyan = vec3(0.0, 0.9, 1.0);
        vec3 neon_pink = vec3(1.0, 0.05, 0.6);
        vec3 neon_purple = vec3(0.5, 0.1, 1.0);

        vec3 line_color = mix(neon_cyan, neon_pink, sin(p.y * 0.05 + color_shift * 6.28) * 0.5 + 0.5);
        line_color = mix(line_color, neon_purple, sin(p.x * 0.1) * 0.5 + 0.5);

        // Add glow to lines
        col += line_color * grid_val * fog * 1.5 * glow_mult;
        // Subtle ambient floor fill
        col += line_color * 0.08 * fog * glow_mult;
    } else {
        // Sky above horizon
        float sky_y = uv.y - horizon;
        
        // Neon Horizon Glow
        float horizon_glow = exp(-sky_y * 14.0) * 1.2 * glow_mult;
        vec3 h_color = mix(vec3(1.0, 0.1, 0.5), vec3(0.0, 0.8, 1.0), sin(t * 0.3) * 0.5 + 0.5);
        col += h_color * horizon_glow;

        // Digital sun/core above horizon
        vec2 sun_pos = vec2(0.0, horizon + 0.28);
        float sun_dist = length(uv - sun_pos);
        if (sun_dist < 0.22) {
            // Horizontal synth sun stripes
            float stripes = sin((uv.y - sun_pos.y) * 90.0);
            if (stripes > -0.2 || (uv.y - sun_pos.y) > 0.08) {
                float sun_grad = (uv.y - sun_pos.y + 0.22) / 0.44;
                vec3 sun_col = mix(vec3(1.0, 0.05, 0.4), vec3(1.0, 0.9, 0.2), sun_grad);
                col += sun_col * 1.8 * glow_mult;
            }
        }
        // Sun outer atmospheric glow
        col += vec3(1.0, 0.2, 0.6) * exp(-sun_dist * 4.5) * 0.45 * glow_mult;

        // Distant cyber stars
        vec2 star_uv = uv * 20.0;
        float n = fract(sin(dot(floor(star_uv), vec2(127.1, 311.7))) * 43758.5453);
        if (n > 0.96) {
            col += vec3(0.7, 0.85, 1.0) * (sin(t * 2.0 + n * 20.0) * 0.5 + 0.5) * 0.7;
        }
    }

    // Gentle vignette
    float vig = 1.0 - length(v_uv - 0.5) * 0.75;
    col *= clamp(vig, 0.25, 1.0);

    fragColor = vec4(col, 1.0);
}
"#
    }
}
