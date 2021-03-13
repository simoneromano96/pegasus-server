mod configuration;
mod graphql;
mod types;
mod utils;

use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::Schema;
use async_graphql::{
	http::{playground_source, GraphQLPlaygroundConfig},
	EmptySubscription,
};
use async_graphql_actix_web::{Request, Response};
use configuration::init_logger;
use graphql::{Mutation, MySchema, Query};
use types::AppContext;
use utils::init_database;

// struct SubscriptionRoot;
//
// #[Subscription]
// impl SubscriptionRoot {
//     async fn values(&self, ctx: &Context<'_>) -> async_graphql::Result<impl Stream<Item = i32>> {
//         if ctx.data::<MyToken>()?.0 != "123456" {
//             return Err("Forbidden".into());
//         }
//         Ok(stream::once(async move { 10 }))
//     }
// }

async fn index(schema: web::Data<MySchema>, _req: HttpRequest, gql_request: Request) -> Response {
	schema.execute(gql_request.into_inner()).await.into()
}

async fn gql_playgound() -> HttpResponse {
	HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(playground_source(
			GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"),
		))
}

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

	let db = init_database().await;

	let context = AppContext { db };

	let schema = Schema::build(Query::default(), Mutation::default(), EmptySubscription)
		.data(context)
		.finish();

	// trace!("Tracing test");
	// debug!("Debug test");
	// info!("Info test");
	// warn!("Warn test");
	// error!("Error test");

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
