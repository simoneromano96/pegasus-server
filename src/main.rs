mod graphql;
mod utils;

use actix_web::{self, guard, web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{Request, Response};
use graphql::{Mutation, Query, User};
use wither::{mongodb::Client, prelude::*};

type MySchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(schema: web::Data<MySchema>, _req: HttpRequest, gql_request: Request) -> Response {
    let request = gql_request.into_inner();
    schema.execute(request).await.into()
}

async fn gql_playgound() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(
            GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
        ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Connect
    let db = Client::with_uri_str("mongodb://root:example@localhost:27017/")
        .await
        .expect("Could not connect to the db")
        .database("mydb");

    // Sync indexes
    User::sync(&db).await.expect("Could not sync user indexes");

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db.clone())
        .finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/").guard(guard::Post()).to(index))
            // .service(
            //     web::resource("/")
            //         .guard(guard::Get())
            //         .guard(guard::Header("upgrade", "websocket"))
            //         .to(index_ws),
            // )
            .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
