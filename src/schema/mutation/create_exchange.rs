use super::{steps::Step, Mutation};
use juniper::{graphql_object, FieldResult, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLInputObject, Debug)]
pub struct CreateExchangeInput {
    #[graphql(description = "API authentication key")]
    auth: Option<String>,
    #[graphql(description = "The steps the Image will go through")]
    steps: Option<Vec<Step>>,
    #[graphql(description = "End behavior")]
    end_behavior: Option<EndBehaviorInput>,
}

#[derive(GraphQLInputObject, Debug)]
pub struct EndBehaviorInput {
    post: EndPostInput,
}

#[derive(GraphQLInputObject, Debug)]
pub struct EndPostInput {
    url: String,
}

#[derive(GraphQLObject, Debug)]
pub struct CreateExchangeReturn {
    upload_link: String,
    download_link: String,
}

#[graphql_object]
impl Mutation {
    fn create_exchange(input: CreateExchangeInput) -> FieldResult<CreateExchangeReturn> {
        println!("{:#?}", input);

        Ok(CreateExchangeReturn {
            upload_link: "Epic".to_owned(),
            download_link: "gamer".to_owned(),
        })
    }
}
