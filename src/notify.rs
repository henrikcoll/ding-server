mod discord_webhook;
mod email;

use crate::config::{load_config, User};

pub async fn notify(user: User, message: Option<String>) {
	let config = load_config();
	match user.discord_webhook {
		Some(account) => discord_webhook::notify(&account, &message).await,
		None => {}
	};

	match config.email {
		Some(email_config) => match user.email {
			Some(account) => email::notify(email_config, &account, &message),
			None => {}
		},
		None => {}
	}
}
