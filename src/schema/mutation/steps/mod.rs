use juniper::{ GraphQLObject};

#[derive(GraphQLObject)]
pub struct Crop {
    width: i32,
    height: i32,
}

pub enum StepMethods {
    Crop(Crop),
}

graphql_union!(Character: () |&self| {
    instance_resolvers: |_| {
        &Crop => match *self { StepMethod::Crop(ref h) => Some(h), _ => None },
    }
});
