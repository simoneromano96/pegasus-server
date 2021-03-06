mod configuration;

use actix_web::{guard, web, App, HttpRequest, HttpResponse, HttpServer};
use async_graphql::{
	http::{playground_source, GraphQLPlaygroundConfig},
	EmptySubscription,
};
use async_graphql::{Context, Data, EmptyMutation, Object, Result, Schema, Subscription};
use async_graphql_actix_web::{Request, Response, WSSubscription};
use configuration::logger::LOGGER;
use slog::{debug, info};

type MySchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

struct QueryRoot;

#[Object]
impl QueryRoot {
	async fn hello(&self) -> Result<String> {
		Ok(String::from("Hello world"))
	}

	/// Gets a password associated with an URL, or will fail if not found
	async fn get_password(&self, url: String) -> Result<String> {
		debug!(LOGGER, "get_password {}", url);
		let password = "password123";
		let result = format!("The password of: {} is {}", url, password);
		Ok(result)
	}
}

struct MutationRoot;

#[Object]
impl MutationRoot {
	/// Saves a password associated with an URL
	async fn set_password(&self, url: String, password: String) -> Result<String> {
		debug!(LOGGER, "set_password {} {}", url, password);
		Ok(format!("Saved {} password {} successfully!", url, password))
	}
}

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

async fn index(schema: web::Data<MySchema>, req: HttpRequest, gql_request: Request) -> Response {
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
	let schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription);

	info!(LOGGER, "Playground: http://localhost:8000");

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
