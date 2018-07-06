//! "Constants" that are shared between systems as resources. They might not actually be constant,
//! but they serve the purpose that you would usually consider a constant.

#[derive(Copy, Clone, Default, Debug)]
pub struct BaseMovementSpeed(pub i8);
