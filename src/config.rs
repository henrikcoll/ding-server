use figment::{
	providers::{Env, Format, Toml},
	Figment,
};

use rocket::serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DiscordWebhook {
	pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct Email {
	pub email: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
	pub username: String,
	pub secret: String,
	pub discord_webhook: Option<DiscordWebhook>,
	pub email: Option<Email>,
}

#[derive(Deserialize, Debug)]
pub struct EmailConfig {
	pub from: String,
	pub username: String,
	pub password: String,
	pub server: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
	pub users: Vec<User>,
	pub email: Option<EmailConfig>,
}

pub fn load_config() -> Config {
	Figment::new()
		.merge(Toml::file("Ding.toml"))
		.merge(Env::prefixed("DING_"))
		.extract()
		.unwrap()
}
