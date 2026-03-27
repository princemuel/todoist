pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use axum::Router;
use axum::routing::get;

pub fn views(router: Router) -> Router { serve(router) }

fn serve(router: Router) -> Router {
    router.nest("/api/v1", Router::new().route("/tasks", get(get::get_all)))
}
