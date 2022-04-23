use rocket::http::Status;
use rocket::serde::{json::Json, Deserialize};

mod config;
mod notify;

#[macro_use]
extern crate rocket;

#[get("/v1/<secret>")]
async fn ding(secret: String) -> (Status, &'static str) {
	let config = config::load_config();

	for user in config.users {
		if user.secret == secret {
			notify::notify(user, None).await;
			return (Status::Ok, "Ok");
		}
	}

	(Status::NotFound, "Not Found")
}

#[derive(Deserialize)]
struct Message {
	body: Option<String>,
}

#[post("/v1/<secret>", data = "<message>")]
async fn ding_post(secret: String, message: Json<Message>) -> (Status, &'static str) {
	let config = config::load_config();

	for user in config.users {
		if user.secret == secret {
			notify::notify(user, message.body.clone()).await;
			return (Status::Ok, "Ok");
		}
	}

	(Status::NotFound, "Not Found")
}

#[get("/v1")]
fn index() -> String {
	String::from("Hi")
}

#[launch]
fn rocket() -> _ {
	rocket::build().mount("/", routes![index, ding, ding_post])
}
