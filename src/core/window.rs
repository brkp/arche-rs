use crate::ContextBuilder;
use sdl2::{render::Canvas, video::Window, Sdl, VideoSubsystem};

pub struct SDLWindow {
    pub sdl: Sdl,
    pub video_subsystem: VideoSubsystem,
    pub canvas: Canvas<Window>,
}

impl SDLWindow {
    pub fn new(builder: &ContextBuilder) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        let window = video_subsystem
            .window(&builder.title, builder.width as u32, builder.height as u32)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

        Ok(Self {
            sdl,
            video_subsystem,
            canvas,
        })
    }
}
