use std::{error::Error, io, time::Duration};

use crossterm::{
    event::{self, poll, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use log::error;
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Span, Text},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};

use crate::{
    sniff::local_mac,
    structs::{
        arp::ARPOperation,
        net::{Device, MacAddr},
    },
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
            match packet.operation {
                ARPOperation::Request => {}
                ARPOperation::Reply => {
                    let dev = Device {
                        mac: packet.sender_mac,
                        ip: packet.sender_ip,
                    };
                    if let Some(already_existing) = app.list.get_by_mac(&dev.mac) {
                        // Command::new("sh")
                        //     .args([
                        //         "-c",
                        //         "notify-send",
                        //         "ARP Watch",
                        //         format!(
                        //             "[{}] @ {} -> [{}] {}",
                        //             already_existing.mac, already_existing.ip, dev.mac, dev.ip
                        //         )
                        //         .as_str(),
                        //     ])
                        //     .exec();
                        if already_existing.ip != dev.ip {
                            app.changement_list
                                .items
                                .push((already_existing.clone(), dev));
                        }
                    } else {
                        app.list.items.push(dev);
                    }
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
    let header_chunk = Rect::new(0, 0, f.size().width, 3);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)].as_ref())
        .split(Rect::new(0, 3, f.size().width, f.size().height - 3));
    f.render_widget(header(app.arp_frame_counter, mac), header_chunk);
    f.render_stateful_widget(
        render_list(app.list.items.clone(), |item| item.to_string()),
        chunks[0],
        &mut app.list.state,
    );
    f.render_stateful_widget(
        render_list(app.changement_list.items.clone(), |item| {
            format!("{} -> {}", item.0, item.1)
        }),
        chunks[1],
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
