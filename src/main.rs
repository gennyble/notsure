use std::str::FromStr;

use confindent::Confindent;
use glow::HasContext;
use glutin::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::unix::WindowBuilderExtUnix,
    window::{Window, WindowBuilder},
    ContextBuilder, ContextWrapper, PossiblyCurrent,
};
use thiserror::Error;

fn main() {
    let _notsure = NotSure::run();
}

struct NotSure {
    context: ContextWrapper<PossiblyCurrent, Window>,
    gl: glow::Context,
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

        let gl = unsafe {
            glow::Context::from_loader_function(|s| context.get_proc_address(s) as *const _)
        };

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

        unsafe {
            let cc = config.clear_color;
            self.gl.clear_color(cc.r, cc.g, cc.b, cc.a);
        }

        self.config = Some(config);
    }

    pub fn draw(&self) {
        unsafe {
            self.gl.clear(glow::COLOR_BUFFER_BIT);
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
                unsafe {
                    self.gl
                        .viewport(0, 0, physical.width as i32, physical.height as i32)
                };
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

#[derive(Copy, Clone, Debug)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Color {
    pub fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b, a: 1.0 }
    }

    pub fn rgba(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn grey(v: f32) -> Self {
        Self {
            r: v,
            g: v,
            b: v,
            a: v,
        }
    }
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<f32> = s
            .split(',')
            .map(|s| s.trim().parse())
            .collect::<Result<_, _>>()
            .map_err(|_e| ColorParseError::InvalidColor(s.into()))?;

        match numbers.len() {
            1 => Ok(Self::grey(numbers[0])),
            3 => Ok(Self::rgb(numbers[0], numbers[1], numbers[2])),
            4 => Ok(Self::rgba(numbers[0], numbers[1], numbers[2], numbers[3])),
            _ => Err(ColorParseError::InvalidColor(s.into())),
        }
    }
}

#[derive(Debug, Error)]
enum ColorParseError {
    #[error("The color {0} could not be parsed")]
    InvalidColor(String),
}
