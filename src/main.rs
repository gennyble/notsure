mod color;
mod gl;
mod vec2;

use std::{cell::RefCell, cmp::Ordering, rc::Rc};

use color::Color;
use gl::{OpenGl, Texture};
pub use vec2::Vec2;

use confindent::Confindent;
use glutin::{
    dpi::PhysicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowBuilderExtUnix,
    window::{Window, WindowBuilder},
    ContextBuilder, ContextWrapper, PossiblyCurrent,
};

/// OpenGL's normalized coordinates are relative to each axis, which would make
/// sizing something like a square quite difficult. So we have our own unit of
/// measurement, Murs.
pub struct Transform {
    pub dpi_scale: f32,
    pub physical_size: PhysicalSize<u32>,
    pub physical_vec: Vec2,
    pub mur_axis_size: u32,
    pub mur_size: u32,
}

impl Transform {
    /// The mur_size is the number of pixels per Mur.
    pub fn new(physical_size: PhysicalSize<u32>, mur_size: u32) -> Self {
        let mur_axis_size = match physical_size.width.cmp(&physical_size.height) {
            Ordering::Equal | Ordering::Greater => physical_size.width,
            Ordering::Less => physical_size.height,
        };

        Self {
            dpi_scale: 1.0,
            physical_size,
            physical_vec: physical_size.into(),
            mur_axis_size,
            mur_size,
        }
    }

    pub fn vec_to_opengl(&self, vec: Vec2) -> Vec2 {
        (vec * self.mur_size) / (self.physical_vec / 2)
    }
}

fn main() {
    let command = std::env::args().nth(1);

    match command.as_deref() {
        Some("server") => todo!(),
        Some("client") | None => NotSure::run(),
        Some(cmd) => eprintln!("'{}' is not a thing, silly :3", cmd),
    }
}

struct NotSure {
    context: ContextWrapper<PossiblyCurrent, Window>,
    gl: OpenGl,
    config: Option<Config>,
    transform: Rc<RefCell<Transform>>,

    beescream: Texture,
}

impl NotSure {
    pub fn run() -> ! {
        let window_size = PhysicalSize::new(640, 240);

        let el = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title("notsure")
            .with_app_id("pleasefloat".into())
            .with_inner_size(window_size);
        let wc = ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &el)
            .unwrap();
        let context = unsafe { wc.make_current().unwrap() };

        let transform = Transform::new(window_size, 320);
        let wrapped_transform = Rc::new(RefCell::new(transform));

        let gl = OpenGl::new(&context, wrapped_transform.clone());
        let beescream = Texture::from_file(&gl, "images/eightytest.png");

        println!("Setup OpenGL!");

        let mut ns = Self {
            gl,
            context,
            config: None,
            transform: wrapped_transform,

            beescream,
        };
        ns.load_config();

        println!("Just about to run!");

        el.run(move |event, _, flow| ns.event_handler(event, flow))
    }

    pub fn load_config(&mut self) {
        let config = Config::load();

        self.gl.clear_color(config.clear_color);

        self.config = Some(config);
    }

    pub fn draw(&self) {
        unsafe {
            self.gl.clear();

            self.beescream.bind(&self.gl);
            for x in 0..8 {
                self.gl
                    .draw_rectangle(((x as f32 - 4.0) * 0.25, 0.125).into(), (0.25, 0.25).into());
            }
        }
    }

    pub fn event_handler(&mut self, event: Event<()>, flow: &mut ControlFlow) {
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => self.window_event(event, flow),
            Event::RedrawRequested(_) => self.context.swap_buffers().unwrap(),
            Event::MainEventsCleared => {
                self.draw();
                self.context.swap_buffers().unwrap();
            }
            _ => (),
        }
    }

    pub fn window_event(&mut self, event: WindowEvent, flow: &mut ControlFlow) {
        match event {
            WindowEvent::Resized(physical) => {
                self.transform.borrow_mut().physical_size = physical;
                self.context.resize(physical);
                self.gl.viewport(physical.width, physical.height);
            }
            WindowEvent::KeyboardInput { input, .. } => {
                if let Some(VirtualKeyCode::Escape) = input.virtual_keycode {
                    *flow = ControlFlow::Exit;
                } else if let Some(VirtualKeyCode::R) = input.virtual_keycode {
                    self.load_config();
                }
            }
            _ => (),
        }
    }
}

struct Config {
    pub clear_color: Color,
}

impl Config {
    pub fn load() -> Self {
        let conf = Confindent::from_file("notsure.conf").unwrap();
        let clear_color = conf.child_parse("ClearColor").unwrap();

        Self { clear_color }
    }
}
