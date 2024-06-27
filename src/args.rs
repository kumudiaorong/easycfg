use clap::Parser;

#[derive(Parser)]
#[command(name = "Easy Config")]
#[command(version = "1.0")]
#[command(about = "easy config new pc", long_about = None)]
pub struct Args {
    #[arg(short, long, default_value = "./")]
    pub directory: String,
}
