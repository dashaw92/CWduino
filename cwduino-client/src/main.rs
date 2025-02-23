mod app;
mod imp;

use std::io::Result;

use app::start_tui;

fn main() -> Result<()> {
    let terminal = ratatui::init();
    start_tui(terminal)?;
    ratatui::restore();

    Ok(())
}
