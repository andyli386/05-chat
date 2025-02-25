use axum::response::IntoResponse;

pub(crate) async fn list_chat_handler() -> impl IntoResponse {
    "chat"
}

pub(crate) async fn create_chat_handler() -> impl IntoResponse {
    "create"
}

pub(crate) async fn update_chat_handler() -> impl IntoResponse {
    "update"
}

pub(crate) async fn delete_chat_handler() -> impl IntoResponse {
    "delete"
}
