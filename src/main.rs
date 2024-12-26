use std::io;
use crossterm::{event::{self, Event, KeyCode, EnableMouseCapture, DisableMouseCapture}, execute};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen};
use tui::{
    backend::{CrosstermBackend, Backend},
    layout::{Alignment, Constraint, Direction, Layout},
    widgets::{Block, BorderType, Borders},
    Terminal
};

const PATH_MAX: usize = 4096;

#[link(name = "getcwd")]
extern "C" {
    fn sys_get_cwd(buffer: *mut u8, size: usize) -> usize;
}

pub fn get_cwd() -> Result<String, std::io::Error> {
    let mut buffer = vec![0u8; PATH_MAX];
    let length = unsafe { sys_get_cwd(buffer.as_mut_ptr(), buffer.len())};

    if length <= 0 {
        return Err(std::io::Error::last_os_error());
    };

    let cwd = String::from_utf8_lossy(&buffer[..length]);
    Ok(cwd.to_string())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, &get_cwd()?);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    if let Err(e) = res {
        println!("Error: {:?}", e);
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, cwd: &String) -> io::Result<()> {
    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(90), Constraint::Percentage(10)].as_ref())
                .split(f.size());

            let horizontal_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(75), Constraint::Percentage(25)].as_ref())
                .split(chunks[0]);

            let block = Block::default()
                .borders(Borders::ALL)
                .title(cwd.to_string())
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded);
            f.render_widget(block, horizontal_chunks[0]);

            let block = Block::default()
                .borders(Borders::ALL)
                .title("info")
                .title_alignment(Alignment::Center)
                .border_type(BorderType::Rounded);
            f.render_widget(block, horizontal_chunks[1]);

            let block = Block::default()
                .borders(Borders::ALL)
                .title("Actions")
                .title_alignment(Alignment::Left)
                .border_type(BorderType::Rounded);
            f.render_widget(block, chunks[1]);
        })?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}
