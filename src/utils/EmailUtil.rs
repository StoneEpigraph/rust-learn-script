use std::fmt::Error;
use lettre::{Message, SmtpTransport, Transport};
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;

fn send_mail(from: &str, to: &str, subject: &str, body: &str) -> Result<String, String>{
    let email = Message::builder()
        .from(from.parse().unwrap())
        .to(to.parse().unwrap())
        .subject(subject)
        .body(body.to_owned())
        .unwrap();

    let creds = Credentials::new("stoneepigraph".to_owned(), "EFREGFBAIWNHCEMP".to_owned());

    let mailer = SmtpTransport::relay("smtp.163.com")
        .unwrap()
        .credentials(creds)
        .build();

    match mailer.send(&email) {
        Ok(_) => {
            Ok("Email Sent successfully!!!".to_owned())
        },
        Err(error) => {
            Err("Could not send email: {error:?}".to_owned())
        },
    }
}

#[test]
pub fn test_send_email() {
    let from = "WhatsUpeng!!!<stoneepigraph@163.com>";
    let to = "StoneEpigraph<stoneepigraph@163.com>";
    let subject = "hello from lettre!";
    let body = "This is a test email from rust use lettre crate";

    assert_eq!(Ok("Email Sent successfully!!!"), send_mail(from, to, subject, body).as_deref())
}