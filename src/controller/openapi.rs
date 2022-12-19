use crate::common;
use crate::controller::user_controller;
use crate::model::User;
use actix_web::web;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        user_controller::save,
        user_controller::update
    ),
    components(
        schemas(User, common::ApiResult<User>, common::ApiResultErr)
    ),
    tags(
        (name = "user", description = "User management endpoints.")
    )
)]
pub struct ApiDoc;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()),
    );
}
