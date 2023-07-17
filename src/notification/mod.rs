use lettre::email::EmailBuilder;
use lettre::transport::smtp::{SecurityLevel, SmtpTransportBuilder};


async fn send_email_smtp(
    body: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let to_address = "hello@example.com";
    let smtp_server = "smtp.googlemail.com";
    let smtp_username = "exampleaccount@gmail";
    let smtp_password = "hunter2";
    let smtp_port = 587u16;

    let email = EmailBuilder::new()
        .to(to_address)
        .from(smtp_username)
        .subject("I am contacting you in respect of a family treasure of Gold deposited in my name")
        .body("i am Becki Ofori a Ghanian from Ashanti region Kumasi, Ghana.")
        .build().unwrap();

    let mut mailer = SmtpTransportBuilder::new((smtp_server, smtp_port)).unwrap()
        .hello_name("localhost")
        .credentials(smtp_username, smtp_password)
        .security_level(SecurityLevel::AlwaysEncrypt)
        .smtp_utf8(true)
        .build();

    mailer.send(email.clone())
}