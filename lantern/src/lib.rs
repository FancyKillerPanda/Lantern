pub mod application;
pub mod events;
pub mod window_base;

/// Represents a size
/// 
/// The two fields are a `width` and a `height`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Size(pub u32, pub u32);

/// Represents a position
/// 
/// The two fields are an `x` and a `y`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position(pub u32, pub u32);

/// Represents an offset
/// 
/// The two fields are an `xOffset` and a `yOffset`
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Offset(pub u32, pub u32);
