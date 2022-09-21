use sdl2::pixels::PixelFormatEnum;

use crate::core::state::{State, StateManager};
use crate::{core::SDLWindow, Texture, Trans};

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
    // maximized
    // minimized
}

pub struct Context {
    pub config: ContextBuilder,
    pub window: SDLWindow,
    pub texture: Texture,
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

    pub fn build(self) -> Result<Context, String> {
        Context::new(self)
    }
}

impl Context {
    pub fn new(config: ContextBuilder) -> Result<Self, String> {
        let window = SDLWindow::new(&config)?;
        let texture = Texture::new(config.width, config.height);

        Ok(Self {
            config,
            window,
            texture,
        })
    }

    fn draw(&mut self, sdl_texture: &mut sdl2::render::Texture) -> Result<(), String> {
        sdl_texture
            .with_lock(None, |buffer: &mut [u8], _pitch: usize| {
                buffer.copy_from_slice(&self.texture.pixel);
            })
            .map_err(|e| e.to_string())?;

        self.window.canvas.clear();
        self.window
            .canvas
            .copy(sdl_texture, None, None)
            .map_err(|e| e.to_string())?;
        self.window.canvas.present();

        Ok(())
    }

    pub fn run(&mut self, initial_state: Box<dyn State>) -> Result<(), String> {
        // assuming that there is no other instance of `EventPump`
        let mut event_pump = self.window.sdl.event_pump().unwrap();
        let mut state_manager = StateManager::with(initial_state);

        let texture_creator = self.window.canvas.texture_creator();
        let mut sdl_texture = texture_creator
            .create_texture_streaming(
                PixelFormatEnum::ARGB32,
                self.config.width as u32,
                self.config.height as u32,
            )
            .map_err(|e| e.to_string())?;

        let mut scan = Vec::<std::time::Duration>::new();
        'outer: while state_manager.get_state_count() > 0 {
            let start = std::time::Instant::now();

            let state = state_manager.get_active_state().unwrap();
            for event in event_pump.poll_iter() {
                match state.handle_event(self, event) {
                    Trans::Pop => {
                        state_manager.pop_state();
                        continue 'outer;
                    }
                    Trans::Set(new_state) => {
                        state_manager.set_state(new_state);
                        continue 'outer;
                    }
                    Trans::Push(new_state) => {
                        state_manager.push_state(new_state);
                        continue 'outer;
                    }
                    Trans::Quit => {
                        let duration = scan.iter().sum::<std::time::Duration>();
                        println!("total scan time: {:?}", duration);
                        println!("frame scan time: {:?}", duration / scan.len() as u32);
                        println!(
                            "scan frame rate: {} fps",
                            1000000 / (duration / scan.len() as u32).as_micros()
                        );
                        break 'outer;
                    }
                    Trans::None => (),
                }
            }

            state.update(self);
            state.draw(self);

            self.draw(&mut sdl_texture)?;
            scan.push(std::time::Instant::now() - start);
        }

        Ok(())
    }
}
