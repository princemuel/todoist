pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use axum::Router;
use axum::routing::get;

use crate::actions::delete::delete as delete_by_name;
use crate::actions::get::get_by_name;

pub fn views(router: Router) -> Router { serve(router) }

fn serve(router: Router) -> Router {
    router.nest(
        "/api/v1",
        Router::new()
            .route(
                "/tasks",
                get(get::get_all).post(create::create).patch(update::update),
            )
            .route("/tasks/{name}", get(get_by_name).delete(delete_by_name)),
    )
}
