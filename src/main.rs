use ratatui::{DefaultTerminal, Frame};
use ratatui_ascii_art::{AsciiBuilder, AsciiImage};

fn main() -> std::io::Result<()> {
    ratatui::run(app)?;
    Ok(())
}

fn app(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    let ascii_art = AsciiImage::default().fit();

    loop {
        terminal.draw(|frame: &mut Frame| {
            frame.render_widget(&ascii_art, frame.area());
        })?;

        if crossterm::event::poll(std::time::Duration::from_millis(41))? {
            if crossterm::event::read()?.is_key_press() {
                break Ok(());
            };
        };
    }
}
