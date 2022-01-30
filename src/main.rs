mod color;
mod gl;
mod vec2;

use color::Color;
use gl::{OpenGl, Texture};
pub use vec2::Vec2;

use confindent::Confindent;
use glutin::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowBuilderExtUnix,
    window::{Window, WindowBuilder},
    ContextBuilder, ContextWrapper, PossiblyCurrent,
};

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
    dimensions: Vec2,
    beescream: Texture,
}

impl NotSure {
    pub fn run() -> ! {
        let el = EventLoop::new();
        let wb = WindowBuilder::new()
            .with_title("notsure")
            .with_app_id("pleasefloat".into())
            .with_inner_size(LogicalSize::new(640.0, 480.0));
        let wc = ContextBuilder::new()
            .with_vsync(true)
            .build_windowed(wb, &el)
            .unwrap();
        let context = unsafe { wc.make_current().unwrap() };

        let gl = OpenGl::new(&context);
        let beescream = Texture::from_file(&gl, "images/beescream.png");

        println!("Setup OpenGL!");

        let mut ns = Self {
            gl,
            context,
            config: None,
            dimensions: (640.0, 480.0).into(),
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
            self.gl
                .draw_rectangle((0.0, 0.0).into(), self.tenth_height_square());
        }
    }

    pub fn tenth_height_square(&self) -> Vec2 {
        let tenth_height = self.dimensions.y / 10.0 / self.dimensions.y;
        let tenth_height_width = self.dimensions.y / 10.0 / self.dimensions.x;

        (tenth_height_width, tenth_height).into()
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
                self.context.resize(physical);
                self.dimensions = (physical.width as f32, physical.height as f32).into();
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
