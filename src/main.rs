use anyhow::Result;
use clap::Parser;
use easycfg::{config, server, tui::Tui};
use tracing::info;
use tracing_subscriber::{filter::LevelFilter, EnvFilter};

fn setup_log() -> Result<()> {
    // let file = std::fs::OpenOptions::new()
    //     .create(true)
    //     .append(true)
    //     .open("./ecfg.log")?;
    let filter: EnvFilter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()?;
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();
    Ok(())
}

fn main() -> Result<()> {
    setup_log()?;
    let args = easycfg::args::Args::parse();
    info!("easycfg directory: {}", args.directory);
    let server_builder = server::ServerBuilder::new();
    info!("distribution: {}", server_builder.distri);
    let (cfg, cwd) = config::init(args.directory)?;
    let server = server_builder.current_dir(cwd).build(cfg)?;
    let mut tui = Tui::new(server);
    tui.run()?;
    Ok(())
}
