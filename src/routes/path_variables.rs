use axum::extract::Path;

pub async fn path_variables(Path((user_id, team_id)): Path<(i32, i32)>) -> String {
    format!("User id :{}, Team id :{}", user_id, team_id)
}
