use glow::{Buffer, HasContext, VertexArray};

use crate::Vec2;

pub struct Rectangle {
    vao: VertexArray,
    vbo: Buffer,
}

impl Rectangle {
    pub fn new(gl: &glow::Context, size: Vec2) -> Self {
        let hx = size.x / 2.0;
        let hy = size.y / 2.0;

        #[rustfmt::skip]
        let verticies = [
            // Top-left triangle
            -hx, hy, 0.0,
            hx, hy, 0.0,
            -hx, -hy, 0.0,

            // Bottom-right triangle
            hx, hy, 0.0,
            hx, -hy, 0.0,
            -hx, -hy, 0.0
        ];

        let mut verticie_buffer = vec![];
        for vertex in verticies {
            verticie_buffer.extend_from_slice(&vertex.to_le_bytes());
        }

        let (vao, vbo) = unsafe {
            let vao = gl.create_vertex_array().unwrap();
            gl.bind_vertex_array(Some(vao));

            let vbo = gl.create_buffer().unwrap();
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &verticie_buffer, glow::DYNAMIC_DRAW);

            gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 12, 0);
            gl.enable_vertex_attrib_array(0);

            gl.bind_buffer(glow::ARRAY_BUFFER, None);
            gl.bind_vertex_array(None);

            (vao, vbo)
        };

        Self { vao, vbo }
    }

    pub unsafe fn bind(&self, gl: &glow::Context) {
        gl.bind_vertex_array(Some(self.vao));
    }
}
