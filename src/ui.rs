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

use crate::{
    sniff::local_mac,
    structs::net::{Device, MacAddr},
    App,
};

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
    let local_mac = local_mac()?;
    loop {
        if let Ok(packet) = app.rx.try_recv() {
            let target = Device {
                mac: packet.target_mac,
                ip: packet.target_ip,
            };

            let broadcast_mac = MacAddr::new(&[00, 00, 00, 00, 00, 00]).unwrap();
            if app.list.has_same_mac(&target) {
                if let Some(already_existing) = app.list.get(&target.mac) {
                    if already_existing.ip != target.ip {
                        app.changement_list
                            .items
                            .push((already_existing.clone(), target));
                    }
                }
            } else {
                if target.mac != broadcast_mac {
                    app.list.items.push(target);
                }
            }
            app.arp_frame_counter += 1;
        }

        term.draw(|f| ui(f, &mut app, local_mac.clone()))?;
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

fn ui<B: Backend>(f: &mut Frame<B>, app: &mut App, mac: MacAddr) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(40),
                Constraint::Percentage(40),
            ]
            .as_ref(),
        )
        .split(f.size());
    f.render_widget(header(app.arp_frame_counter, mac), chunks[0]);
    f.render_stateful_widget(
        render_list(app.list.items.clone(), |item| item.to_string()),
        chunks[1],
        &mut app.list.state,
    );
    f.render_stateful_widget(
        render_list(app.changement_list.items.clone(), |item| {
            format!("{} -> {}", item.0, item.1)
        }),
        chunks[2],
        &mut app.changement_list.state,
    );
}

fn render_list<T, F>(list: Vec<T>, f: F) -> List<'static>
where
    F: Fn(&T) -> String,
{
    let items = list
        .iter()
        .map(|item| ListItem::new(Span::raw(f(item))))
        .collect::<Vec<ListItem>>();
    List::new(items)
        .highlight_style(Style::default().fg(Color::Black).bg(Color::White))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
}

fn header(frame_count: usize, mac: MacAddr) -> Paragraph<'static> {
    Paragraph::new(Text::raw(format!(
        "ARP Watch [{}] (Frame: {})",
        mac, frame_count
    )))
    .alignment(Alignment::Center)
    .block(
        Block::default()
            .borders(Borders::all())
            .border_type(tui::widgets::BorderType::Double),
    )
}
