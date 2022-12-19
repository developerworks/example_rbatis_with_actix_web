use lombok::AllArgsConstructor;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, AllArgsConstructor, ToSchema)]
pub struct ApiResult<'a, T> {
    pub code: u16,
    pub message: &'a str,
    pub data: T,
}
#[derive(Debug, Clone, Serialize, AllArgsConstructor, ToSchema)]
pub struct ApiResultErr<'a> {
    pub code: u16,
    pub message: &'a str,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: rbatis::Rbatis,
}
