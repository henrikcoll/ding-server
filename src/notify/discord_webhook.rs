use crate::config::DiscordWebhook;
use reqwest::Client;
use rocket::serde::Serialize;

#[derive(Serialize, Debug)]
struct DiscordWebhookBody {
	content: String,
}

pub async fn notify(webhook: &DiscordWebhook, message: &Option<String>) {
	let client = Client::new();
	let body = DiscordWebhookBody {
		content: match message.clone() {
			Some(message) => message,
			None => String::from("Ding!"),
		},
	};
	let res = client.post(webhook.url.clone()).json(&body).send().await;
}
