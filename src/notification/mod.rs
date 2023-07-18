use std::env;

use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message,
    message::header::ContentType, Tokio1Executor, transport::smtp::authentication::Credentials,
};
use lettre::transport::smtp::Error;
use lettre::transport::smtp::response::Response;

pub(crate) async fn send_email_smtp(
    body: &str
) -> Result<Response, Error> {
    let to_address = env::var("DINING_ALERT_EMAIL").expect("DINING_ALERT_EMAIL not set");
    let smtp_server = env::var("DINING_SMTP_SERVER").expect("DINING_SMTP_SERVER not set");
    let smtp_username = env::var("DINING_SMTP_USERNAME").expect("DINING_SMTP_USERNAME not set");
    let smtp_password = env::var("DINING_SMTP_PASSWORD").expect("DINING_SMTP_PASSWORD not set");
    let smtp_port = env::var("DINING_SMTP_PORT").expect("DINING_SMTP_PORT");

    let email = Message::builder()
        .from(smtp_username.parse().unwrap())
        .to(to_address.parse().unwrap())
        .subject("Dining Alert: Reservation found!")
        .header(ContentType::TEXT_PLAIN)
        .body(String::from(body))
        .unwrap();

    let creds = Credentials::new(smtp_username.to_owned(), smtp_password.to_owned());

    let mailer: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(smtp_server.as_str())
            .unwrap()
            .port(smtp_port.parse().unwrap())
            .credentials(creds)
            .build();

    mailer.send(email).await
}