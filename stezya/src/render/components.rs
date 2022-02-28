use std::time::Duration;

#[derive(Copy, Clone, Debug, Default)]
pub struct RenderDelta(pub Duration);

pub const DEF_WINDOW_WIDTH: u32 = 1496;
pub const DEF_WINDOW_HEIGHT: u32 = 1024;

#[derive(Copy, Clone, Debug)]
pub struct WindowSize(pub u32, pub u32);

impl Default for WindowSize {
    fn default() -> Self {
        WindowSize(DEF_WINDOW_WIDTH, DEF_WINDOW_HEIGHT)
    }
}
