use crate::{Trans, Texture, core::Window};
use crate::core::state::{State, StateManager};

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
    pub window: Window,
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
        let window = Window::new(&config)?;
        let texture = Texture::new(config.width, config.height);

        Ok(Self {
            config,
            window,
            texture,
        })
    }

    pub fn run(&mut self, initial_state: Box<dyn State>) {
        let mut state_manager = StateManager::with(initial_state);
        // assuming that there is no other instance of `EventPump`
        let mut event_pump = self.window.sdl.event_pump().unwrap();

        'outer: while state_manager.get_state_count() > 0 {
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
                    Trans::Quit => break 'outer,
                    Trans::None => ()
                }
            }

            state.update(self);
            state.draw(self);
        }
    }
}
