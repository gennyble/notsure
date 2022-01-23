mod color;
mod gl;
mod vec2;

use color::Color;
use gl::OpenGl;
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
    let _notsure = NotSure::run();
}

struct NotSure {
    context: ContextWrapper<PossiblyCurrent, Window>,
    gl: OpenGl,
    config: Option<Config>,
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

        let mut ns = Self {
            gl,
            context,
            config: None,
        };
        ns.load_config();

        el.run(move |event, _, flow| ns.event_handler(event, flow))
    }

    pub fn load_config(&mut self) {
        let config = Config::load();

        unsafe { self.gl.clear_color(config.clear_color) }

        self.config = Some(config);
    }

    pub fn draw(&self) {
        unsafe {
            self.gl.clear();
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
                self.context.resize(physical);
                unsafe { self.gl.viewport(physical.width, physical.height) };
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
