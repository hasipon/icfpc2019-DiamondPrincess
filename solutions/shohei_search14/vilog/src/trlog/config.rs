pub struct TrlogConfig {
    pub fps: f32
}

impl TrlogConfig {
    pub fn new() -> TrlogConfig {
        TrlogConfig {
            fps: 15.0
        }
    }
}
