#![feature(test)]
extern crate test;

use std::error::Error;

use ratatui::{
    backend::{Backend, TestBackend},
    layout::Rect,
    Terminal,
    widgets::Paragraph,
};

#[test]
fn terminal_buffer_size_should_be_limited() {
    let backend = TestBackend::new(400, 400);
    let terminal = Terminal::new(backend).unwrap();
    let size = terminal.backend().size().unwrap();
    assert_eq!(size.width, 255);
    assert_eq!(size.height, 255);
}

#[test]
fn terminal_draw_returns_the_completed_frame() -> Result<(), Box<dyn Error>> {
    let backend = TestBackend::new(10, 10);
    let mut terminal = Terminal::new(backend)?;
    let frame = terminal.draw(|f| {
        let paragraph = Paragraph::new("Test");
        f.render_widget(paragraph, f.size());
    })?;
    assert_eq!(frame.buffer.get(0, 0).symbol, "T");
    assert_eq!(frame.area, Rect::new(0, 0, 10, 10));
    terminal.backend_mut().resize(8, 8);
    let frame = terminal.draw(|f| {
        let paragraph = Paragraph::new("test");
        f.render_widget(paragraph, f.size());
    })?;
    assert_eq!(frame.buffer.get(0, 0).symbol, "t");
    assert_eq!(frame.area, Rect::new(0, 0, 8, 8));
    Ok(())
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use ratatui::backend::CrosstermBackend;
    use ratatui::backend::TermionBackend;

    use super::*;

    #[bench]
    fn crossterm_create_terminal(b: &mut Bencher) {
        let stdout = std::io::stdout();
        b.iter(|| {
            let backend = CrosstermBackend::new(&stdout);
            let _ = Terminal::new(backend);
        });
    }

    #[cfg(feature = "termion")]
    #[bench]
    fn termion_create_terminal(b: &mut Bencher) {
        let stdout = std::io::stdout();
        b.iter(|| {
            let backend = TermionBackend::new(&stdout);
            let _ = Terminal::new(backend);
        });
    }

    #[bench]
    fn terminal_draw(b: &mut Bencher) {
        let backend = TestBackend::new(10, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        b.iter(|| {
            terminal.draw(|f| {
                let paragraph = Paragraph::new("Test");
                f.render_widget(paragraph, f.size());
            }).unwrap();
        });
    }
}
