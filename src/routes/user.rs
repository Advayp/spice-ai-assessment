use rocket::serde::json::Json;

use crate::{
    database::{self, user::AuthData},
    models::user::UserInfo,
};

#[post("/user/signup", data = "<auth>")]
pub async fn sign_up(auth: Json<AuthData>) -> Json<UserInfo> {
    let email = auth.clone().into_inner().email;
    let password = auth.into_inner().password;

    let res = database::user::sign_up(email, password).await.unwrap();

    Json(res.json::<UserInfo>().await.unwrap())
}

#[post("/user/login", data = "<auth>")]
pub async fn login(auth: Json<AuthData>) -> Json<UserInfo> {
    let email = auth.clone().into_inner().email;
    let password = auth.into_inner().password;

    let res = database::user::login(email, password).await.unwrap();

    Json(res.json::<UserInfo>().await.unwrap())
}
