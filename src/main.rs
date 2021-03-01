mod configuration;
mod graphql;
mod types;
mod utils;

use std::{io::Result, sync::Arc};

use actix_web::{self, guard, web, App, HttpMessage, HttpRequest, HttpResponse, HttpServer};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{Request, Response};
use configuration::APP_CONFIG;
use graphql::{Mutation, Query, User};
use types::{AppContext, UserSession};
use utils::{init_database, init_redis_client, redis_deserialize_get};

type MySchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(
    schema: web::Data<MySchema>,
    // app_context: web::Data<AppContext>,
    redis: web::Data<Arc<redis::Client>>,
    req: HttpRequest,
    gql_request: Request,
) -> Response {
    let mut request = gql_request.into_inner();
    let mut user_session = None;
    if let Some(cookie) = req.cookie(&APP_CONFIG.cookie.name) {
        let session_id = cookie.value();
        if let Ok(user) = redis_deserialize_get::<User>(&redis, session_id).await {
            user_session = Some(UserSession {
                user,
                session_id: session_id.to_string(),
            });
        }
    }
    request = request.data(user_session);
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
async fn main() -> Result<()> {
    let db = init_database().await;
    // Redis
    let redis = Arc::new(init_redis_client());
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
