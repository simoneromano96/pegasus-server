mod configuration;
mod graphql;
mod utils;

use std::sync::Arc;

use actix_web::{self, guard, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{Request, Response};
use configuration::APP_CONFIG;
use graphql::{Mutation, Query, User};
use redis::AsyncCommands;
use wither::{mongodb::Client, prelude::*};

struct AppContext {
    pub db: wither::mongodb::Database,
    pub redis: Arc<redis::Client>,
}

type MySchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(
    schema: web::Data<MySchema>,
    // app_context: web::Data<AppContext>,
    redis: web::Data<Arc<redis::Client>>,
    req: HttpRequest,
    gql_request: Request,
) -> Response {
    let request = gql_request.into_inner();
    if let Some(cookie) = req.cookie(&APP_CONFIG.cookie.name) {
        // println!("{:?}", cookie);
        let session_id = cookie.value();
        println!("{}", session_id);
        let mut redis_connection = redis.get_async_connection().await.expect("fuck");
        let username: Option<String> = redis_connection.get(session_id).await.expect("fuck 2");
        println!("{:?}", username);
    }
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

    // Redis
    let redis = Arc::new(redis::Client::open("redis://127.0.0.1/").expect("Could not init redis"));

    let app_context = AppContext {
        db,
        redis: redis.clone(),
    };

    let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(app_context)
        .finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .data(redis.clone())
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
