use crate::constants::message_constants;
use crate::db::user::User;
use crate::db::CrmDbConn;
use crate::models::response::Response;
use crate::proxies::naive_date_form_proxy::NaiveDateForm;
use crate::schema::user_auth_token;
use chrono::{NaiveDateTime, Utc};
use diesel::PgConnection;
use diesel::{Insertable, Queryable};
use jsonwebtoken::errors::Result;
use jsonwebtoken::{Algorithm, TokenData};
use jsonwebtoken::{DecodingKey, EncodingKey};
use jsonwebtoken::{Header, Validation};
use rocket::http::Status;
use rocket::request;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "user_auth_token"]
pub struct UserAuthToken {
    pub user_id: i32,
    pub login_session: String,
    pub generated_at: NaiveDateForm,
    pub expires_at: NaiveDateForm,
}

const ONE_WEEK: i64 = 60 * 60 * 24 * 7;

#[derive(Serialize, Deserialize, FromForm)]
pub struct LoginInfo {
    username_or_email: String,
    password: String,
}

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
                if let Ok(token_data) = decode_token(token.to_string()) {
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
    pub fn generate_auth_token(user_id: i32, login_session: &str) -> UserAuthToken {
        let now = Utc::now().naive_utc();
        let expiry_secs = now.timestamp() as i64 + ONE_WEEK;
        let expiry = NaiveDateTime::from_timestamp(expiry_secs, 0);
        UserAuthToken {
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

fn decode_token(token: String) -> Result<TokenData<UserAuthToken>> {
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
        &token,
        &DecodingKey::from_secret(include_bytes!("secret.key")),
        &jwt_validation,
    )
}

fn verify_token(token_data: &TokenData<UserAuthToken>, conn: &PgConnection) -> bool {
    User::is_valid_login_session(&token_data.claims, conn)
}
