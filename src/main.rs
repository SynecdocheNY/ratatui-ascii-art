use ratatui::{DefaultTerminal, Frame};

fn main() -> std::io::Result<()> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let ascii_art = ratatui_ascii_art::AsciiImage::default();

    loop {
        terminal.draw(|frame: &mut Frame| {
            frame.render_widget(&ascii_art, frame.area());
        })?;

        if crossterm::event::read()?.is_key_press() {
            break Ok(());
        }
    }
}
