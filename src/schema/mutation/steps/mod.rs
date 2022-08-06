use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject, Debug)]
/// Crops the image
pub struct Crop {
    width: i32,
    height: i32,
}

/// Select only one option in step
///
/// Returns Error if more than 1 is selected
#[derive(GraphQLInputObject, Debug)]
pub struct Step {
    crop: Option<Crop>,
}
