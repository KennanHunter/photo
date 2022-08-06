use super::{steps::Steps, Mutation};
use juniper::{graphql_object, FieldResult, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLInputObject)]
#[graphql(description = "")]
pub struct CreateExchangeInput {
    auth: Option<String>,
    step: Option<Vec<Steps>>,
    end_behavior: Option<EndBehaviorInput>,
}

#[derive(GraphQLInputObject)]
pub struct EndBehaviorInput {
    post: EndPostInput,
}

#[derive(GraphQLInputObject)]
pub struct EndPostInput {
    url: String,
}

#[derive(GraphQLObject)]
pub struct CreateExchangeReturn {
    upload_link: String,
    download_link: String,
}

#[graphql_object]
impl Mutation {
    fn create_exchange(input: CreateExchangeInput) -> FieldResult<CreateExchangeReturn> {
        // println!("{:?}", input);
        Ok(CreateExchangeReturn {
            upload_link: "Epic".to_owned(),
            download_link: "gamer".to_owned(),
        })
    }
}
