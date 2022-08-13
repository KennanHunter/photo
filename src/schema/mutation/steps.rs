use juniper::GraphQLInputObject;

use crate::steps::transform::flip::FlipDirection;

#[derive(GraphQLInputObject, Debug)]
/// Crops the image
pub struct Crop {
    pub width: i32,
    pub height: i32,
    pub xoffset: i32,
    pub yoffset: i32,
}

/// Resize image
#[derive(GraphQLInputObject, Debug)]
pub struct Resize {
    pub width: i32,
    pub height: i32,
}

#[derive(GraphQLInputObject, Debug)]
pub struct Flip {
    pub flip_direction: FlipDirection,
}

/// Select only one option in step
///
/// Returns Error if more than 1 is selected
#[derive(GraphQLInputObject, Debug)]
pub struct Step {
    pub crop: Option<Crop>,
    pub resize: Option<Resize>,
    pub flip: Option<Flip>,
}
