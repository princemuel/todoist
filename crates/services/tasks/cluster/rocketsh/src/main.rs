#[macro_use]
extern crate rocket;

use core::net::Ipv4Addr;

use task_dal::migrations::run_migrations;

mod actions;

#[rocket::main]
#[expect(clippy::result_large_err)]
async fn main() -> Result<(), rocket::Error> {
    run_migrations().await;
    let config = rocket::Config {
        port: 8080,
        address: Ipv4Addr::UNSPECIFIED.into(),
        ..Default::default()
    };

    let server = rocket::custom(&config).mount("/api/v1", actions::serve());
    server.launch().await?;
    Ok(())
}
