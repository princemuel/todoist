pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use rocket::Route;

pub fn serve() -> Vec<Route> {
    routes![
        get::get_all,
        get::get_by_name,
        create::create,
        delete::delete_by_name,
        update::update
    ]
}
