use crate::core::Window;
use crate::Texture;

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

    pub fn build(&mut self) -> Context {
        todo!()
    }
}

impl Context {
    pub fn new(config: &ContextBuilder) -> Result<Self, String> {
        let window = Window::new(config)?;
        let texture = Texture::new(config.width, config.height);

        Ok(Self { window, texture })
    }
}
