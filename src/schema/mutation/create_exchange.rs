use crate::{steps::step_to_func, util::extract_image_base64};
use photon_rs::base64_to_image;

use super::{steps::Step, Mutation};
use juniper::{graphql_object, FieldResult, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLInputObject, Debug)]
pub struct CreateExchangeInput {
    /// API authentication key
    auth: Option<String>,
    /// Image represented as base64
    image_base64: Option<String>,
    #[graphql(description = "The steps the Image will go through")]
    steps: Option<Vec<Step>>,
    #[graphql(description = "End behavior")]
    end_behavior: Option<EndBehaviorInput>,
}

#[derive(GraphQLInputObject, Debug)]
pub struct EndBehaviorInput {
    /// Url to post to when done
    callback: EndPostInput,
}

#[derive(GraphQLInputObject, Debug)]
pub struct EndPostInput {
    url: String,
}

#[derive(GraphQLObject, Debug)]
pub struct CreateExchangeReturn {
    upload_link: Option<String>,
    download_link: Option<String>,
    base64image: Option<String>,
}

#[graphql_object]
impl Mutation {
    fn create_exchange(input: CreateExchangeInput) -> FieldResult<CreateExchangeReturn> {
        // if input.auth.is_none() {
        //     return simple_error("Epic");
        // }

        // if input.image_base64.is_none() {
        //     return simple_error("Fuck");
        // }

        // println!("{:#?}", input.image_base64.as_ref().unwrap());

        let mut image = base64_to_image(&extract_image_base64(&input.image_base64.unwrap()));
        if input.steps.is_some() {
            for step in input.steps.unwrap() {
                image = step_to_func(step, &mut image).unwrap();
            }
        }

        Ok(CreateExchangeReturn {
            upload_link: None,
            download_link: None,
            base64image: Some(image.get_base64()),
        })
    }
}
