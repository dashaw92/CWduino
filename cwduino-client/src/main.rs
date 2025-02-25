mod app;
mod imp;
mod profile;

use std::io::Result;

use app::start_tui;
use clap::Parser;

fn main() -> Result<()> {
    let args = AppArgs::parse();
    let terminal = ratatui::init();
    start_tui(terminal, args)?;
    ratatui::restore();

    Ok(())
}

#[derive(Parser, Debug)]
#[command(version)]
pub(crate) struct AppArgs {
    #[arg(short, long)]
    device: String,
    #[arg(short, long)]
    profile: String,
}
