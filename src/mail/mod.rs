use std::io::BufReader;

use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport
};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MailService
{
    from_address:String,
    smtp_url:String,
    smtp_port:u16,
    user:String,
    password:String
}

const MAIL_SERVICE_FILE:&str = "./secrets/email.json";

pub fn get_service()->Result<MailService,Box::<dyn std::error::Error>>
{
    let file = std::fs::File::open(MAIL_SERVICE_FILE)?;
    let service = serde_json::from_reader(BufReader::new(file))?;
    Ok(service)
}

pub fn send_mail(
    service: &MailService,
    recipient: &str,
    subject: String,
    body: String,
) -> Result<lettre::transport::smtp::response::Response, Box<dyn std::error::Error>> {
    let sender :Mailbox= service.from_address.parse()?;

    let email = Message::builder()
        .from(sender.clone())
        .reply_to(sender)
        .to(recipient.parse()?)
        .subject(subject)
        .body(body)?;

    // Create TLS transport on port 465
    let sender = SmtpTransport::relay(&service.smtp_url)?
        .port(service.smtp_port)
        .credentials(Credentials::new(
            service.user.to_string(),
            service.password.to_string(),
        ))
        .build();

    // Send the email via remote relay
    Ok(sender.send(&email)?)
}
