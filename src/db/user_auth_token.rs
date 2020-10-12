use crate::constants::message_constants;
use crate::db::CrmDbConn;
use crate::models::response::Response;
use crate::models::user::User;
use crate::models::user_auth_token::{UserAuthToken, ONE_WEEK};
use crate::proxies::naive_date_form_proxy::NaiveDateForm;
use chrono::{NaiveDateTime, Utc};
use diesel::PgConnection;
use jsonwebtoken::errors::Result;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::response::status;
use rocket_contrib::json::Json;

impl<'a, 'r> FromRequest<'a, 'r> for UserAuthToken {
    type Error = status::Custom<Json<Response>>;
    fn from_request(
        request: &'a Request<'r>,
    ) -> request::Outcome<Self, status::Custom<Json<Response>>> {
        let conn = request.guard::<CrmDbConn>().unwrap();
        if let Some(authen_header) = request.headers().get_one("Authorization") {
            let authen_str = authen_header.to_string();
            if authen_str.starts_with("Bearer") {
                let token = authen_str[6..authen_str.len()].trim();
                if let Ok(token_data) = decode_token(token) {
                    if verify_token(&token_data, &conn.0) {
                        return Outcome::Success(token_data.claims);
                    }
                }
            }
        }

        Outcome::Failure((
            Status::BadRequest,
            status::Custom(
                Status::Unauthorized,
                Json(Response {
                    message: String::from(message_constants::MESSAGE_INVALID_TOKEN),
                    data: serde_json::to_value("").unwrap(),
                }),
            ),
        ))
    }
}

impl UserAuthToken {
    pub fn generate_auth_token(user_id: i32, login_session: &str) -> Self {
        let now = Utc::now().naive_utc();
        let expiry_secs = now.timestamp() as i64 + ONE_WEEK;
        let expiry = NaiveDateTime::from_timestamp(expiry_secs, 0);
        Self {
            user_id,
            login_session: login_session.to_string(),
            generated_at: NaiveDateForm::new(now),
            expires_at: NaiveDateForm::new(expiry),
        }
    }
}

pub fn generate_jwt_token(auth_token: &UserAuthToken) -> String {
    jsonwebtoken::encode(
        &Header::default(),
        auth_token,
        &EncodingKey::from_secret(include_bytes!("secret.key")),
    )
    .unwrap()
}

fn decode_token(token: &str) -> Result<TokenData<UserAuthToken>> {
    let jwt_validation: Validation = Validation {
        leeway: 0,
        validate_exp: false,
        validate_nbf: false,
        aud: None,
        iss: None,
        sub: None,
        algorithms: vec![Algorithm::HS256],
    };
    jsonwebtoken::decode::<UserAuthToken>(
        token,
        &DecodingKey::from_secret(include_bytes!("secret.key")),
        &jwt_validation,
    )
}

fn verify_token(token_data: &TokenData<UserAuthToken>, conn: &PgConnection) -> bool {
    User::is_valid_login_session(&token_data.claims, conn)
}
