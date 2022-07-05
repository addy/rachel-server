use super::super::models::contact::{Contact, ContactResponse, Message};
use async_trait::async_trait;
use axum::extract::Json;
use lettre::{
    message::{header, MultiPart, SinglePart},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Message as LettreMessage, Tokio1Executor,
};
use std::sync::Arc;

pub type DynContactService = Arc<dyn ContactService + Send + Sync>;

#[async_trait]
pub trait ContactService {
    async fn create_contact(&self, Json(params): Json<Contact>) -> ContactResponse;
}

pub struct ContactServiceImpl {
    host: String,
    user: String,
    pass: String,
    email: String,
}

#[async_trait]
impl ContactService for ContactServiceImpl {
    async fn create_contact(&self, Json(params): Json<Contact>) -> ContactResponse {
        let message = Message::new(
            params.first_name.to_string(),
            params.last_name.to_string(),
            params.email.to_string(),
            params.message.to_string(),
        );

        let email = LettreMessage::builder()
            .from(self.user.parse().unwrap())
            .reply_to(String::from(params.email).parse().unwrap())
            .to(self.email.parse().unwrap())
            .subject(message.subject)
            .multipart(
                MultiPart::alternative()
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_PLAIN)
                            .body(message.text),
                    )
                    .singlepart(
                        SinglePart::builder()
                            .header(header::ContentType::TEXT_HTML)
                            .body(message.html),
                    ),
            )
            .unwrap();
        
        let creds = Credentials::new(self.user.to_string(), self.pass.to_string());

        let mailer: AsyncSmtpTransport<Tokio1Executor> =
            AsyncSmtpTransport::<Tokio1Executor>::relay(&self.host)
                .unwrap()
                .credentials(creds)
                .build();
    
        match mailer.send(email).await {
            Ok(_) => ContactResponse { success: true },
            Err(_) => ContactResponse { success: false },
        }
    }
}

impl ContactServiceImpl {
    pub fn new() -> Self {
        let host = std::env::var("SMTP_HOST").expect("SMTP_HOST must be set");
        let user = std::env::var("SMTP_USER").expect("SMTP_USER must be set");
        let pass = std::env::var("SMTP_PASS").expect("SMTP_PASS must be set");
        let email = std::env::var("TO_EMAIL").expect("TO_EMAIL must be set");

        Self {
            host,
            user,
            pass,
            email,
        }
    }
}
