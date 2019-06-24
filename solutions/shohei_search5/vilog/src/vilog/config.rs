pub struct VilogConfig {
    pub width: u32,
    pub height: u32,
    pub background: u32,
    pub fps: f32
}

impl VilogConfig {
    pub fn new(width:u32, height:u32) -> VilogConfig {
        VilogConfig {
            width,
            height,
            background: 0xFFFFFFFF,
            fps: 4.0
        }
    }
}
