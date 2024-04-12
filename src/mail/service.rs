use axum::http::StatusCode;
use lettre::{
    message::{header::ContentType, Mailbox},
    transport::smtp::authentication::Credentials,
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
};

use crate::app::{envy::Envy, models::api_error::ApiError};

pub async fn send(to: &str, subject: &str, body: &str, envy: &Envy) -> Result<(), ApiError> {
    let Ok(mailbox) = to.parse::<Mailbox>() else {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Invalid recipient address.",
        ));
    };
    let Ok(from) = envy.mail_from.parse::<Mailbox>() else {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "Invalid sender address.",
        ));
    };

    let Ok(mail) = lettre::Message::builder()
        .to(mailbox)
        .from(from)
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(String::from(body))
    else {
        return Err(ApiError::internal_server_error());
    };

    let credentials = Credentials::new(envy.mail_user.to_string(), envy.mail_pass.to_string());
    let mailer = AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(&envy.mail_host)
        .unwrap()
        .port(envy.mail_port)
        .credentials(credentials)
        .build();

    match mailer.send(mail).await {
        Ok(_) => Ok(()),
        Err(e) => {
            tracing::error!(%e);
            Err(ApiError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to send mail.",
            ))
        }
    }
}
