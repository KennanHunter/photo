use juniper::{
    graphql_object, FieldResult, GraphQLEnum, GraphQLInputObject, GraphQLObject, Nullable,
};

use super::Mutation;

#[derive(GraphQLInputObject)]
#[graphql(description = "")]
pub struct CreateExchangeInput {
    auth: Nullable<String>,
    step: Nullable<Vec<StepInput>>,
    end_behavior: Nullable<EndBehaviorInput>,
}

#[derive(GraphQLInputObject)]
pub struct EndBehaviorInput {
    post: EndPostInput,
}

#[derive(GraphQLInputObject)]
pub struct EndPostInput {
    url: String,
}

#[derive(GraphQLInputObject)]
pub struct StepInput {
    method: steps::StepMethod,
}

#[derive(GraphQLObject)]
pub struct CreateExchangeReturn {
    upload_link: String,
    download_link: String,
}

#[graphql_object]
impl Mutation {
    fn create_exchange(input: CreateExchangeInput) -> FieldResult<CreateExchangeReturn> {
        Ok(CreateExchangeReturn {
            upload_link: "Epic".to_owned(),
            download_link: "Epic2".to_owned(),
        })
    }
}
