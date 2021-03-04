mod configuration;
mod graphql;
mod types;
mod utils;

use std::{io::Result, sync::Arc};

use actix_web::{self, guard, web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_actix_web::{Request, Response};
use graphql::{Mutation, Query};
use redis::Client as RedisClient;
use types::AppContext;
use utils::{get_session, init_database, init_redis_client};
use web::Data;

use crate::configuration::APP_CONFIG;

type MySchema = Schema<Query, Mutation, EmptySubscription>;

async fn index(
    schema: Data<MySchema>,
    // app_context: web::Data<AppContext>,
    redis: Data<Arc<RedisClient>>,
    req: HttpRequest,
    gql_request: Request,
) -> Response {
    let mut request = gql_request.into_inner();
    let user_session = get_session(req, &redis).await;
    if let Some(session) = user_session {
        request = request.data(session);
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
    .bind(format!("0.0.0.0:{}", &APP_CONFIG.server.port))?
    .run()
    .await
}
