mod graphql;

use actix_web::{self, guard, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{Context, Data, EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::{Request, Response};
use graphql::User;
use graphql::UserQuery;
use std::sync::Arc;
use wither::{mongodb::Client, prelude::*};

type MySchema = Schema<UserQuery, EmptyMutation, EmptySubscription>;

async fn index(schema: web::Data<MySchema>, req: HttpRequest, gql_request: Request) -> Response {
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
    let db = Arc::new(
        Client::with_uri_str("mongodb://root:example@localhost:27017/")
            .await
            .expect("Could not connect to the db")
            .database("mydb"),
    );
    // Sync indexes
    User::sync(&db)
        .await
        .expect("Could not sync user indexes");

    let schema = Schema::build(UserQuery, EmptyMutation, EmptySubscription)
        .data(db)
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
