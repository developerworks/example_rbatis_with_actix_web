use actix_web::http::StatusCode;
use actix_web::patch;
use actix_web::{dev::HttpServiceFactory, post, web, HttpResponse, Responder, Result};

use crate::common::{ApiResult, ApiResultErr, AppState};
use crate::model::User;
use rbatis::rbdc::db::ExecResult;

rbatis::crud!(User {});

#[post("/save")]
async fn save(state: web::Data<AppState>, body: web::Json<User>) -> Result<impl Responder> {
    let mut db = &state.pool.clone();
    let mut user = body.to_owned();
    let result: ExecResult = User::insert(&mut db, &user).await.unwrap();
    user.set_id(result.last_insert_id.as_u64());

    // TODO:: 返回值用 Either
    let response = HttpResponse::Ok()
        .status(StatusCode::CREATED)
        .json(ApiResult::new(0, "", user));
    Ok(response)
}

#[patch("/{id}")]
async fn update(state: web::Data<AppState>, body: web::Json<User>) -> Result<impl Responder> {
    let db = &mut state.pool.clone();
    let user = body.to_owned();

    let result: ExecResult = User::update_by_column(db, &user, "id").await.unwrap();
    let response = if result.rows_affected == 1 {
        HttpResponse::Ok().json(ApiResult::new(0, "", user))
    } else {
        let s = format!("The updated resource {} not found", user.id.unwrap());
        HttpResponse::Ok()
            .status(StatusCode::GONE)
            .json(ApiResultErr::new(0, &s))
    };
    Ok(response)
}

pub fn api_routes() -> impl HttpServiceFactory {
    web::scope("/user").service(save).service(update)
}
