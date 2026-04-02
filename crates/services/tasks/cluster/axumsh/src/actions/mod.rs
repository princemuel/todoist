pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use axum::Router;
use axum::routing::get;
use task_dal::tasks::descriptors::SqlxPostgresDescriptor;

use crate::actions::delete::delete as delete_by_name;
use crate::actions::get::get_by_name;

pub fn views(router: Router) -> Router { serve(router) }

fn serve(router: Router) -> Router {
    router.nest(
        "/api/v1",
        Router::new()
            .route(
                "/tasks",
                get(get::get_all::<SqlxPostgresDescriptor>)
                    .post(create::create::<SqlxPostgresDescriptor>)
                    .patch(update::update::<SqlxPostgresDescriptor>),
            )
            .route(
                "/tasks/{name}",
                get(get_by_name::<SqlxPostgresDescriptor>)
                    .delete(delete_by_name::<SqlxPostgresDescriptor>),
            ),
    )
}
