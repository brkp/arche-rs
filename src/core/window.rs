use sdl2;
use crate::core::ContextBuilder;

pub struct Window {
    // TODO: this might not have to be pub at all
    pub sdl: sdl2::Sdl,
    pub video_subsystem: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,
    pub event_pump: sdl2::EventPump,
}

impl Window {
    pub fn new(builder: &ContextBuilder) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        let window = video_subsystem
            .window(&builder.title, builder.width as u32, builder.height as u32)
            .position_centered()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let event_pump = sdl.event_pump()?;
        
        Ok(Self {
            sdl ,
            video_subsystem,
            window,
            event_pump
        })
    }
}
