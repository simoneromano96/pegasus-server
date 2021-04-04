mod configuration;
mod graphql;
mod types;
mod utils;

use actix_web::{self, guard, web, App, HttpRequest, HttpServer};
use async_graphql::{EmptySubscription, Schema};
use async_graphql_actix_web::{Request, Response};
use configuration::init_logger;
use graphql::{Mutation, MySchema, Query};
use log::debug;
use redis::Client as RedisClient;
use types::AppContext;
use utils::{get_session, init_database, init_redis_client};
use web::Data;

use crate::configuration::APP_CONFIG;

async fn index(
  schema: Data<MySchema>,
  // app_context: web::Data<AppContext>,
  redis: Data<RedisClient>,
  req: HttpRequest,
  gql_request: Request,
) -> Response {
  // Get the GQL Request
  let mut request = gql_request.into_inner();

  // Get the user session and add it to the context
  let user_session = get_session(&redis, req).await;
  debug!("(Maybe) user session: {:?}", &user_session);

  if let Some(session) = user_session {
    request = request.data(session);
    debug!("Added user session to Context");
  }

  schema.execute(request).await.into()
}

// async fn gql_playgound() -> HttpResponse {
//   HttpResponse::Ok()
//     .content_type("text/html; charset=utf-8")
//     .body(playground_source(
//       GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
//     ))
// }
// async fn index_ws(
//     schema: web::Data<MySchema>,
//     req: HttpRequest,
//     payload: web::Payload,
// ) -> Result<HttpResponse> {
//     WSSubscription::start_with_initializer(Schema::clone(&*schema), &req, payload, |value| async {
//         #[derive(serde_derive::Deserialize)]
//         struct Payload {
//             token: String,
//         }
//
//         if let Ok(payload) = serde_json::from_value::<Payload>(value) {
//             let mut data = Data::default();
//             data.insert(MyToken(payload.token));
//             Ok(data)
//         } else {
//             Err("Token is required".into())
//         }
//     })
// }

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  init_logger();

  // Mongo database
  let db = init_database()
    .await
    .expect("Could not initialise database!");

  // Redis client
  let redis = init_redis_client().expect("Could not initialise redis!");

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
    // .service(web::resource("/").guard(guard::Get()).to(gql_playgound))
  })
  .bind(format!("0.0.0.0:{}", &APP_CONFIG.app.port))?
  .run()
  .await
}
