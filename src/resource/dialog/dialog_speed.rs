#[derive(Copy, Clone, Debug)]
pub struct DialogSpeed(pub f32);

impl Default for DialogSpeed {
    fn default() -> Self {
        DialogSpeed(1f32)
    }
}
