use lettre::{
    transport::smtp::authentication::{Credentials, Mechanism},
    Message, SmtpTransport,
};
use std::env;

const AUTOSCHEDA_BOT: &str = "Autoscheda Bot <bot@autoscheda.com>";
const ZOHO_SMTP_URL: &str = "smtp.zoho.com";
const ZOHO_SMTP_PORT: u16 = 465;
const ZOHO_APP_UNAME: &str = "ZOHO_APP_UNAME";
const ZOHO_APP_PW: &str = "ZOHO_APP_PW";

pub fn send_mail(
    recipient: &str,
    subject: String,
    body: String,
) -> Result<_, Box<dyn std::error::Error>> {
    let sender = AUTOSCHEDA_BOT.parse()?;

    let email = Message::builder()
        .from(sender)
        .reply_to(sender)
        .to(recipient.parse()?)
        .subject(subject)
        .body(body)?;

    // Create TLS transport on port 465
    let sender = SmtpTransport::relay(ZOHO_SMTP_URL)?
        .port(ZOHO_SMTP_PORT)
        .credentials(Credentials::new(
            env::var(ZOHO_APP_UNAME),
            env::var(ZOHO_APP_PW),
        ))
        .build();
    // Send the email via remote relay
    let result = sender.send(&email);

    Ok(result)
}
