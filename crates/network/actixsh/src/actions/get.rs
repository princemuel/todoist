use actix_web::HttpResponse;
use engine::actions::get::get_all as get_all_tasks;

pub async fn get_all() -> HttpResponse {
    let tasks = match get_all_tasks() {
        Ok(items) => items,
        Err(e) => return HttpResponse::InternalServerError().json(e),
    };
    HttpResponse::Ok().json(tasks)
}
