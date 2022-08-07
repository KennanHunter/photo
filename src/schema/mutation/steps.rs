use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject, Debug)]
/// Crops the image
pub struct Crop {
    pub width: i32,
    pub height: i32,
    pub xoffset: i32,
    pub yoffset: i32,
}

/// Select only one option in step
///
/// Returns Error if more than 1 is selected
#[derive(GraphQLInputObject, Debug)]
pub struct Step {
    pub crop: Option<Crop>,
}
