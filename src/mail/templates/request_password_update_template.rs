use crate::app;

pub fn new(access_token: &str) -> (String, String) {
    let url = format!(
        "{}/auth/password/{}",
        app::config::FRONTEND_URL,
        access_token
    );

    (
        format!("{} password update", app::config::APP_NAME),
        format!(
            "
            <p>Hello there!</p>
            <p>We heard that you want to update your {} password.</p>
            <p>You can use the following link to change it:</p>
            <a href={}>{}</a>
            <p>This link will expire in 1 hour.</p>
            <p>If you did not request this, ignore this email.</p>
            <p>Your friends at {}</p>
            ",
            app::config::APP_NAME,
            url,
            url,
            app::config::APP_NAME
        ),
    )
}
