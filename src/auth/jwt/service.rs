use jsonwebtoken::{encode, EncodingKey, Header};

use crate::{app::util::time, auth::models::claims::Claims, users::models::user::User};

use super::config::JWT_EXP;

pub fn sign_jwt(user: &User, secret: &str, pepper: Option<&str>) -> String {
    let mut secret = secret.to_string();
    let iat = time::current_time_in_secs() as u64;
    let exp = iat + JWT_EXP;

    let claims = Claims {
        id: user.id.to_string(),
        iat,
        exp,
    };
    if let Some(pepper) = pepper {
        secret = [&secret, pepper].concat();
    }

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}
