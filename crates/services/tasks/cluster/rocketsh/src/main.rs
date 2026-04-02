#[macro_use]
extern crate rocket;

use core::net::Ipv4Addr;

mod actions;

#[rocket::main]
#[allow(clippy::result_large_err)]
async fn main() -> Result<(), rocket::Error> {
    let config = rocket::Config {
        port: 8080,
        address: Ipv4Addr::UNSPECIFIED.into(),
        ..Default::default()
    };

    let server = rocket::custom(&config).mount("/api/v1", actions::serve());
    server.launch().await?;
    Ok(())
}
