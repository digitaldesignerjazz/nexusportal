//! NexusPortal - The interactive terminal console to the Nova ecosystem
//! Full menu with Grok Launcher integration

use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};
use std::io::stdout;

fn main() -> Result<()> {
    color_eyre::install()?;
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    let mut should_quit = false;

    while !should_quit {
        terminal.draw(|frame| {
            let area = frame.size();

            let title = Paragraph::new("NEXUSPORTAL")
                .style(Style::default().fg(Color::Cyan).bold())
                .alignment(Alignment::Center);

            let subtitle = Paragraph::new("The Door to the Nova Ecosystem  •  Grok Launcher Integrated")
                .style(Style::default().fg(Color::Gray))
                .alignment(Alignment::Center);

            let menu = Paragraph::new(
                "1. Mesh Status          (xMesh / NovaNet / QNET)\n\
                 2. Blockchain           (XCoin / QCoin)\n\
                 3. AI Agent Swarms\n\
                 4. Grok Launcher        ← Launch / Control\n\
                 5. Hardware Prototypes\n\
                 6. Architecture & Docs\n\n\
                 Press Q to quit",
            )
            .block(Block::default().title("Main Menu").borders(Borders::ALL));

            frame.render_widget(title, Rect::new(area.x, area.y + 1, area.width, 1));
            frame.render_widget(subtitle, Rect::new(area.x, area.y + 2, area.width, 1));
            frame.render_widget(menu, Rect::new(area.x + 2, area.y + 5, area.width - 4, area.height - 6));
        })?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => should_quit = true,

                        KeyCode::Char('1') => { /* TODO: Mesh status */ }
                        KeyCode::Char('2') => { /* TODO: Blockchain */ }
                        KeyCode::Char('3') => { /* TODO: Agent swarms */ }

                        KeyCode::Char('4') => {
                            // === Grok Launcher Integration Point ===
                            println!("\n[Grok Launcher] Integration triggered.");
                            // let _ = std::process::Command::new("grok-launcher").spawn();
                        }

                        KeyCode::Char('5') => { /* TODO: Hardware */ }
                        KeyCode::Char('6') => { /* TODO: Show architecture */ }

                        _ => {}
                    }
                }
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}