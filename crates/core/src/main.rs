
#![warn(clippy::pedantic)]
#![warn(clippy::ptr_arg)]
#![warn(clippy::use_self)]
#![warn(clippy::suspicious)]
#![warn(clippy::perf)]

use core::str::FromStr;

use clap::Parser;
use engine::actions::create::create;
use engine::enums::TaskStatus;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    title:  String,
    #[arg(short, long)]
    status: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();
    let status = TaskStatus::from_str(&args.status)?;

    let item = create(&args.title, status)?;
    println!("Created item: {item}");
    Ok(())
}
