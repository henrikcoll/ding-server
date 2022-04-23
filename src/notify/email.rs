extern crate lettre;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::config::Email;
use crate::config::EmailConfig;

pub fn notify(email_config: EmailConfig, email: &Email, message: &Option<String>) {
	let email = Message::builder()
		.from(email_config.from.parse().unwrap())
		.to(email.email.parse().unwrap())
		.subject(match message.clone() {
			Some(message) => format!("ding: {}", message),
			None => String::from("Ding!"),
		})
		.body(String::from("Ding!"))
		.unwrap();

	let creds = Credentials::new(email_config.username, email_config.password);

	let mailer = SmtpTransport::relay(email_config.server.as_str())
		.unwrap()
		.credentials(creds)
		.build();

	let result = mailer.send(&email);
}
