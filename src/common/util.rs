use lombok::AllArgsConstructor;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, AllArgsConstructor)]
pub struct ApiResult<'a, T> {
    pub code: u16,
    pub message: &'a str,
    pub data: T,
}
#[derive(Debug, Clone, Serialize, AllArgsConstructor)]
pub struct ApiResultErr<'a> {
    pub code: u16,
    pub message: &'a str,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: rbatis::Rbatis,
}
