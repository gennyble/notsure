mod color;
mod gl;
mod vec2;

use std::{cell::RefCell, cmp::Ordering, collections::HashMap, path::Path, rc::Rc, str::FromStr};

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
    pub mur_dimensions: Vec2,
    pub mur_half_dimensions: Vec2,
    pub mur_size: u32,
}

impl Transform {
    /// The mur_size is the number of pixels per Mur.
    pub fn new(physical_size: PhysicalSize<u32>, mur_size: u32) -> Self {
        let mur_dimensions = Vec2::from(physical_size) / mur_size;

        Self {
            dpi_scale: 1.0,
            physical_size,
            physical_vec: physical_size.into(),
            mur_half_dimensions: mur_dimensions / 2.0,
            mur_dimensions,
            mur_size,
        }
    }

    pub fn vec_to_opengl(&self, vec: Vec2) -> Vec2 {
        (vec * self.mur_size) / (self.physical_vec / 2)
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Tile {
    Nothing,
    Spawn,
    Air,
    Grass,
    Dirt,
    Ground,
}

struct Gridworld {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Tile>,
}

impl Gridworld {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Self {
        let contents = std::fs::read_to_string(path.as_ref()).unwrap();

        contents.parse().unwrap()
    }

    pub fn find_spawn(&self) -> Option<Vec2> {
        for (idx, tile) in self.grid.iter().enumerate() {
            if *tile == Tile::Spawn {
                let position = Vec2 {
                    x: (idx % self.width) as f32,
                    y: (idx / self.width) as f32,
                };

                return Some(position);
            }
        }

        None
    }
}

impl FromStr for Gridworld {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();

        let mut map: HashMap<char, Tile> = HashMap::new();
        loop {
            match lines.next().map(|s| s.trim()) {
                None => return Err("Did not expect EOF"),
                Some("") => break,
                Some(s) => {
                    let mut chars = s.chars();
                    let indicator = chars.next().ok_or("No indicator in map line?")?;

                    // Throwaway the pipe
                    chars.next();

                    let tile = match chars.collect::<String>().as_ref() {
                        "Spawn" => Tile::Spawn,
                        "Air" => Tile::Air,
                        "Grass" => Tile::Grass,
                        "Dirt" => Tile::Dirt,
                        "Ground" => Tile::Ground,
                        "Nothing" => Tile::Nothing,
                        _ => panic!(),
                    };

                    map.insert(indicator, tile);
                }
            }
        }

        let mut width = 0;
        let mut height = 0;
        let mut grid = vec![];

        for line in lines {
            let chars = line.trim().chars();
            width = line.trim().len();
            height += 1;

            for ch in chars {
                match map.get(&ch) {
                    Some(&tile) => grid.push(tile),
                    None => todo!(),
                }
            }
        }

        Ok(Self {
            width,
            height,
            grid,
        })
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

struct Entity {
    pub texture: Texture,
    pub position: Vec2,
    pub dimensions: Vec2,
}

impl Entity {
    pub fn center(&self) -> Vec2 {
        let mut center = self.position.clone();
        let halfdim = self.dimensions / 2;
        center.x += halfdim.x;
        center.y -= halfdim.y;

        center
    }

    pub fn model_center(&self) -> Vec2 {
        let mut halfdim = self.dimensions / 2;
        halfdim.y *= -1.0;

        halfdim
    }

    pub fn bottom(&self) -> Vec2 {
        let mut pos = self.position;
        pos.y += self.dimensions.y;

        pos
    }
}

struct NotSure {
    context: ContextWrapper<PossiblyCurrent, Window>,
    gl: OpenGl,
    config: Option<Config>,
    transform: Rc<RefCell<Transform>>,

    siva: Entity,
    gridworld: Gridworld,
    tile_textures: HashMap<Tile, Texture>,
    background: Texture,
}

impl NotSure {
    pub fn run() -> ! {
        let window_size = PhysicalSize::new(640, 480);

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

        let transform = Transform::new(window_size, 24);
        let wrapped_transform = Rc::new(RefCell::new(transform));

        let gl = OpenGl::new(&context, wrapped_transform.clone());
        let siva = Entity {
            texture: Texture::from_file(&gl, "images/siva.png"),
            position: (0.0, 0.0).into(),
            dimensions: (1.33, 2.5).into(),
        };
        let background = Texture::from_file(&gl, "images/background.png");
        let gridworld = Gridworld::from_file("test.grid");

        let mut tile_textures = HashMap::new();
        let tile_paths = vec![
            (Tile::Ground, "ground"),
            (Tile::Air, "air"),
            (Tile::Grass, "grass"),
            (Tile::Dirt, "dirt"),
        ];

        for (tile, name) in tile_paths.into_iter() {
            tile_textures.insert(
                tile,
                Texture::from_file(&gl, format!("images/tiles/{}.png", name)),
            );
        }

        println!("Setup OpenGL!");

        let mut ns = Self {
            gl,
            context,
            config: None,
            transform: wrapped_transform,

            siva,
            gridworld,
            tile_textures,
            background,
        };
        ns.load_config();

        match ns.gridworld.find_spawn() {
            None => {
                println!("Could not move siva to spawn!")
            }
            Some(mut spawn) => {
                // Tiles are 1x1 and we want them on the ground
                spawn.y -= 1.0 + ns.siva.dimensions.y;
                ns.siva.position = spawn;
            }
        }

        println!("Just about to run!");

        el.run(move |event, _, flow| ns.event_handler(event, flow))
    }

    pub fn load_config(&mut self) {
        let config = Config::load();

        self.gl.clear_color(config.clear_color);

        self.config = Some(config);
    }

    pub fn draw(&self) {
        let mut camera = self.siva.center();
        camera.y *= -1.0;

        //camera.x = 0.0;
        //camera.y = 0.0;

        unsafe {
            // Draw the background
            self.gl.clear();
            self.background.bind(&self.gl);
            self.gl.draw_fullscreen();

            // Now draw the tile world itself
            for (idx, tile) in self.gridworld.grid.iter().enumerate() {
                if *tile == Tile::Spawn || *tile == Tile::Nothing {
                    continue;
                }

                let x = (idx as f32 % self.gridworld.width as f32).floor();
                let y = (idx as f32 / self.gridworld.width as f32).floor();

                self.tile_textures.get(tile).unwrap().bind(&self.gl);
                self.gl.draw_rectangle(
                    Vec2::new(x, self.gridworld.height as f32 - y) - camera,
                    (1.0, 1.0).into(),
                )
            }

            // Finally, draw siva
            let mut draw = self.siva.model_center() * -1.0;
            draw.y += self.siva.dimensions.y;

            self.siva.texture.bind(&self.gl);
            self.gl.draw_rectangle(draw, self.siva.dimensions);
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
