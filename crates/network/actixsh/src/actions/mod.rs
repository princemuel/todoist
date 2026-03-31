pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use actix_web::web::{ServiceConfig, delete, get, patch, post, scope};

use crate::actions::delete::delete as delete_core;
use crate::actions::get::get_by_name;

pub fn views(app: &mut ServiceConfig) { serve(app); }

fn serve(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
            .route("/tasks", get().to(get::get_all))
            .route("/tasks", post().to(create::create))
            .route("/tasks", patch().to(update::update))
            .route("/tasks/{name}", get().to(get_by_name))
            .route("/tasks/{name}", delete().to(delete_core)),
    );
}
