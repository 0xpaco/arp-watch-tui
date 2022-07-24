use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::error;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::App;

pub fn start_ui(app: App) -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal, app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    if let Err(e) = res {
        error!("UI Crashed {}", e);
    }
    Ok(())
}

fn run_app<B: Backend>(term: &mut Terminal<B>, mut app: App) -> Result<(), Box<dyn Error>> {
    loop {
        // if let Ok(packet) = app.rx.try_recv() {
        //     let (snd_dev, _) = packet.devices();
        //     if !app.list.items.contains(snd_dev) {
        //         app.list.items.push(snd_dev.clone());
        //     }
        // }

        term.draw(|f| ui(f, &mut app))?;
        if poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    _ => (),
                }
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)].as_ref())
        .split(f.size());
    let items = app
        .list
        .items
        .iter()
        .map(|dev| ListItem::new(Span::raw(format!("{}", dev))))
        .collect::<Vec<ListItem>>();
    let list = List::new(items)
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        );
    f.render_widget(header(), chunks[0]);
    f.render_stateful_widget(list, chunks[1], &mut app.list.state);
}

fn header() -> Paragraph<'static> {
    Paragraph::new(Text::raw("ARP Watch"))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::all())
                .border_type(tui::widgets::BorderType::Double),
        )
}
