pub mod context;
pub mod schema;
mod steps;
pub mod util;

use actix_web::{get, route, web, HttpResponse, Responder};
use actix_web_lab::{respond::Html, web::Redirect};
use juniper::http::{graphiql::graphiql_source, GraphQLRequest};

use crate::schema::Schema;

/// GraphiQL playground UI
#[get("/graphiql")]
async fn graphql_playground() -> impl Responder {
    Html(graphiql_source("/graphql", None))
}

/// GraphQL endpoint
#[route("/graphql", method = "GET", method = "POST")]
async fn graphql(st: web::Data<Schema>, data: web::Json<GraphQLRequest>) -> impl Responder {
    let user = data.execute(&st, &()).await;
    HttpResponse::Ok().json(user)
}

#[get("")]
async fn index() -> impl Responder {
    Redirect::new("", "https://kennan.tech")
}
