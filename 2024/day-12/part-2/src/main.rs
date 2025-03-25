pub mod app;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let mut terminal = ratatui::init();
    let app_result = app::App::new().run(&mut terminal);
    ratatui::restore();
    app_result
}

