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

fn get_transport(service: &MailService)->Result<SmtpTransport,Box::<dyn std::error::Error>>
{
    // Create TLS transport on port 465
    let sender = SmtpTransport::relay(&service.smtp_url)?
    .port(service.smtp_port)
    .credentials(Credentials::new(
        service.user.to_string(),
        service.password.to_string(),
    ))
    .build();

    Ok(sender)
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

    let sender = get_transport(service)?;

    // Send the email via remote relay
    Ok(sender.send(&email)?)
}

mod tests {
    use super::*;

    #[test]
    fn test_email_connectivity() {
        let service = get_service().expect("Couldn't get service.");
        let sender = get_transport(&service).expect("Couldnt get transport.");
        sender.test_connection().expect("Connection test failed.");
    }

    #[test]
    fn test_self_email_send() {
        let service = get_service().expect("Couldn't get service.");
        send_mail(
            &service,
            &service.from_address,
            "Block Divider E-mail Test".to_string(),
            "This is a test of programmatic e-mailing.".to_string()
        ).expect("Couldn't send e-mail.");
    }
}