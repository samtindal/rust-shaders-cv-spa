use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlVertexArrayObject};
use wasm_bindgen::JsValue;

pub struct GeometryBuffer {
    vao: Option<WebGlVertexArrayObject>,
    #[allow(dead_code)]
    vbo: Option<WebGlBuffer>,
}

impl GeometryBuffer {
    pub fn new(gl: &WebGl2RenderingContext) -> Result<Self, JsValue> {
        let vao = gl.create_vertex_array();
        gl.bind_vertex_array(vao.as_ref());

        let vbo = gl.create_buffer();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, vbo.as_ref());

        // Full screen quad (2 triangles)
        let vertices: [f32; 12] = [
            -1.0, -1.0,
             1.0, -1.0,
            -1.0,  1.0,
            -1.0,  1.0,
             1.0, -1.0,
             1.0,  1.0,
        ];

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        // Setup vertex attribute pointer (location 0: a_position)
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_with_i32(
            0,
            2,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );

        gl.bind_vertex_array(None);
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);

        Ok(Self { vao, vbo })
    }

    pub fn bind(&self, gl: &WebGl2RenderingContext) {
        gl.bind_vertex_array(self.vao.as_ref());
    }

    pub fn draw(&self, gl: &WebGl2RenderingContext) {
        gl.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, 6);
    }
}

impl Drop for GeometryBuffer {
    fn drop(&mut self) {
        // Resources are cleaned up on context loss or explicit drop
    }
}
