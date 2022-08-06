use juniper::GraphQLInputObject;

#[derive(GraphQLInputObject)]
pub struct Crop {
    width: i32,
    height: i32,
}

#[derive(GraphQLInputObject)]
pub struct Steps {
    crop: Option<Crop>,
}
