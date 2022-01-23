mod rectangle;
mod texture;

pub use rectangle::Rectangle;
pub use texture::Texture;

use std::{path::Path as FilePath, rc::Rc};

use glow::{HasContext, Program};
use glutin::{window::Window, ContextWrapper, PossiblyCurrent};

use crate::{Color, Vec2};

pub struct OpenGl {
    gl: Rc<glow::Context>,
    program: Program,
    clear_color: Color,
    draw_rect: Rectangle,
}

impl OpenGl {
    pub fn new(context: &ContextWrapper<PossiblyCurrent, Window>) -> Self {
        let gl = unsafe {
            glow::Context::from_loader_function(|s| context.get_proc_address(s) as *const _)
        };

        // dirty hack to allow skip on this. Without let _ it would be an expression, and rustfmt::skip
        // on expressions is nightly
        #[rustfmt::skip]
        let _ = unsafe {
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::REPEAT as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::REPEAT as i32);

            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::NEAREST as i32);
            gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::NEAREST as i32);
        };

        let program =
            unsafe { Self::create_program(&gl, "shaders/texture.vert", "shaders/texture.frag") };

        unsafe {
            gl.use_program(Some(program));
        }

        let draw_rect = Rectangle::new(&gl, (2.0, 2.0).into());

        Self {
            gl: Rc::new(gl),
            program,
            clear_color: Color::rgba(0.0, 0.0, 0.0, 1.0),
            draw_rect,
        }
    }

    pub fn gl(&self) -> &glow::Context {
        &self.gl
    }

    pub fn clear_color<C: Into<Color>>(&mut self, color: C) {
        let c = color.into();
        self.clear_color = c;
        unsafe { self.gl.clear_color(c.r, c.g, c.b, c.a) }
    }

    pub fn clear(&self) {
        unsafe { self.gl.clear(glow::COLOR_BUFFER_BIT) }
    }

    pub fn viewport(&self, width: u32, height: u32) {
        unsafe { self.gl.viewport(0, 0, width as i32, height as i32) }
    }

    unsafe fn create_program<V: AsRef<FilePath>, F: AsRef<FilePath>>(
        gl: &glow::Context,
        vertex_path: V,
        fragment_path: F,
    ) -> Program {
        let program = gl.create_program().expect("Failed to create program");

        let shader_soruces = [
            (
                glow::VERTEX_SHADER,
                std::fs::read_to_string(vertex_path).unwrap(),
            ),
            (
                glow::FRAGMENT_SHADER,
                std::fs::read_to_string(fragment_path).unwrap(),
            ),
        ];

        let mut shaders = vec![];
        for (stype, source) in shader_soruces.iter() {
            let shader = gl.create_shader(*stype).expect("Failed to create shader");
            gl.shader_source(shader, source);
            gl.compile_shader(shader);

            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }

            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        // Shaders are compiled and linked with the program, we don't need them anymore
        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        program
    }

    pub fn draw_rectangle(&self, pos: Vec2, dim: Vec2) {
        unsafe {
            //self.gl.use_program(Some(self.program));

            let uniform_position = self.gl.get_uniform_location(self.program, "WorldPosition");
            let uniform_scale = self.gl.get_uniform_location(self.program, "Scale");
            self.gl
                .uniform_2_f32(uniform_position.as_ref(), pos.x, pos.y);
            self.gl.uniform_2_f32(uniform_scale.as_ref(), dim.x, dim.y);

            self.draw_rect.bind(&self.gl);
            self.gl
                .draw_elements(glow::TRIANGLES, 6, glow::UNSIGNED_BYTE, 0);
        }
    }
}

impl Drop for OpenGl {
    fn drop(&mut self) {
        unsafe { self.gl.delete_program(self.program) }
    }
}
