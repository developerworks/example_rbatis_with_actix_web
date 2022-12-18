use lombok::AllArgsConstructor;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, AllArgsConstructor)]
pub struct ApiResult<'a, T> {
    pub code: u16,
    pub message: &'a str,
    pub data: Option<T>,
}

#[derive(Clone)]
pub struct AppState {
    pub pool: rbatis::Rbatis,
}
