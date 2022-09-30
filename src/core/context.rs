use pixels::{Error, Pixels, PixelsBuilder, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use crate::core::state::{State, StateManager};
use crate::{Texture, Trans};

pub struct ContextBuilder {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) title: String,
    // TODO
    // stuff to add:
    // vsync
    // grab mouse
    // show mouse
    // resizable
    // maximized minimized
}

pub struct Context {
    pub config: ContextBuilder,
    pub window: winit::window::Window,
    pub pixels: Pixels,
    pub texture: Texture,
    pub event_loop: Option<EventLoop<()>>,
}

impl ContextBuilder {
    pub fn new(title: String, width: usize, height: usize) -> Self {
        Self {
            title,
            width,
            height,
        }
    }

    pub fn title(&mut self, title: String) -> &mut Self {
        self.title = title;
        self
    }

    pub fn size(&mut self, width: usize, height: usize) -> &mut Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn build(self) -> Result<Context, Error> {
        Context::new(self)
    }
}

impl Context {
    pub fn new(config: ContextBuilder) -> Result<Self, Error> {
        let event_loop = EventLoop::new();
        // TODO: handle errors
        let window = {
            let size = LogicalSize::new(config.width as f64, config.height as f64);
            WindowBuilder::new()
                .with_title(&config.title)
                .with_inner_size(size)
                .with_min_inner_size(size)
                .build(&event_loop)
                .unwrap()
        };
        let texture = Texture::new(config.width, config.height);
        let pixels = {
            let win_size = window.inner_size();
            let surface_texture = SurfaceTexture::new(win_size.width, win_size.height, &window);
            PixelsBuilder::new(config.width as u32, config.height as u32, surface_texture)
                .enable_vsync(false)
                .build()?
        };

        Ok(Self {
            config,
            window,
            pixels,
            texture,
            event_loop: Some(event_loop),
        })
    }

    fn draw(&mut self) {
        // TODO: error handling
        self.pixels.get_frame().copy_from_slice(&self.texture.pixel);
        self.pixels.render().unwrap();
    }

    pub fn run(mut self, initial_state: Box<dyn State>) {
        let mut input_helper = WinitInputHelper::new();
        let mut state_manager = StateManager::with(initial_state);

        let mut scan = Vec::<std::time::Duration>::new();

        self.event_loop.take().unwrap().run(move |event, _, control_flow| {
            control_flow.set_poll();

            if input_helper.update(&event) {
                let start = std::time::Instant::now();
                let mut state = state_manager.get_active_state();
                if state.is_none() {
                    *control_flow = ControlFlow::Exit;
                    return;
                };
                let state = state.take().unwrap();

                match state.handle_events(&mut self, &input_helper) {
                    Trans::Pop => {
                        state_manager.pop_state();
                        return;
                    }
                    Trans::Set(new_state) => {
                        state_manager.set_state(new_state);
                        return;
                    }
                    Trans::Push(new_state) => {
                        state_manager.push_state(new_state);
                        return;
                    }
                    Trans::Quit => {
                        let duration = scan.iter().sum::<std::time::Duration>();
                        println!("total scan time: {:?}", duration);
                        println!("frame scan time: {:?}", duration / scan.len() as u32);
                        println!(
                            "scan frame rate: {} fps",
                            1000000 / (duration / scan.len() as u32).as_micros()
                        );
                        *control_flow = ControlFlow::Exit;
                        return;
                    }
                    Trans::None => (),
                }
                state.update(&mut self);
                state.draw(&mut self);

                self.draw();
                scan.push(std::time::Instant::now() - start);
            }
        });
    }
}
