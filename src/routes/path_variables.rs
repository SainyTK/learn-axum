use axum::extract::Path;

pub async fn path_variables(Path(id): Path<i32>) -> String {
    id.to_string()
}

pub async fn hardcoded_path() -> String {
    "This is hardcoded".to_owned()
}