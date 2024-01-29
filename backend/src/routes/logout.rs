use axum::{http::StatusCode, Extension};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

use crate::utils::decode_access_token;

// add logic if status code == 500 || unauthorized {
// notif(failure)
// }
// else {
// notif(""succesfully logged out"")
// }

async fn delete_refresh_token(
    id: Uuid,
    db: &PgPool,
) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
    let query = "UPDATE users SET refresh_token = $1 WHERE id = $2";

    let result = sqlx::query(query).bind("").bind(id).execute(db).await;
    return result;
}

pub async fn logout(
    Extension(database): Extension<Pool<Postgres>>,
    jar: CookieJar,
) -> Result<CookieJar, StatusCode> {
    let token = jar.get("auth").map(|token| token.value().to_owned());

    match token {
        Some(a_token) => {
            let token_data = decode_access_token(a_token);

            match token_data {
                Ok(data) => {
                    let user_id = data.claims.id;
                    let result = delete_refresh_token(user_id, &database).await;

                    match result {
                        Ok(_) => {
                            //clear cookie
                            let auth_cookie = Cookie::build(("auth", "")).http_only(true);
                            Ok(jar.add(auth_cookie))
                        }
                        Err(error) => {
                            println!("there was an error logging you out: {}", error);
                            Err(StatusCode::INTERNAL_SERVER_ERROR)
                        }
                    }
                }
                Err(error) => {
                    println!("there was an error parsing the auth token : {}", error);
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        None => {
            println!("You are already logged out");
            Err(StatusCode::SERVICE_UNAVAILABLE)
        }
    }
}
