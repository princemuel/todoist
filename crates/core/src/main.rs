use clap::Parser;
use engine::actions::create::create;
use glue::errors::NanoServiceError;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    title:  String,
    #[arg(short, long)]
    status: String,
}

fn main() -> Result<(), NanoServiceError> {
    let args = Args::parse();
    let status = args.status.parse()?;

    let item = create(&args.title, status)?;
    println!("{item}");
    Ok(())
}
