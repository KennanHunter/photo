use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject, Debug)]
pub struct Crop {
    width: i32,
    height: i32,
}

#[derive(GraphQLInputObject, Debug)]
pub struct Steps {
    crop: Option<Crop>,
}
