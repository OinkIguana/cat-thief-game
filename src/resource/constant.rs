//! "Constants" that are shared between systems as resources. They might not actually be constant,
//! but they serve the purpose that you would usually consider a constant.

#[derive(Copy, Clone, Debug)]
pub struct BaseMovementSpeed(pub i8);

impl Default for BaseMovementSpeed {
    fn default() -> Self {
        BaseMovementSpeed(4i8)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct DialogSpeed(pub f32);

impl Default for DialogSpeed {
    fn default() -> Self {
        DialogSpeed(1f32)
    }
}
