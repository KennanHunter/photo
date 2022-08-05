use juniper::FieldResult;

pub struct Query {}

#[juniper::graphql_object]
impl Query {
    fn ok() -> FieldResult<bool> {
        Ok(true)
    }
}
