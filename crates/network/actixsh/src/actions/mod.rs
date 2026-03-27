pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use actix_web::web::{ServiceConfig, get, scope};

pub fn views(app: &mut ServiceConfig) { serve(app); }

fn serve(app: &mut ServiceConfig) {
    app.service(scope("api/v1").route("tasks", get().to(get::get_all)));
}
