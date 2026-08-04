//! Real-time dashboard for monitoring Modbus registers.
//!
//! Provides a terminal UI (TUI) that displays live data from a Modbus register
//! with a sparkline showing the historical trend.

//! Dashboard en tiempo real para monitorear registros Modbus.
//!
//! Proporciona una interfaz de usuario en terminal (TUI) que muestra datos en vivo
//! de un registro Modbus con un sparkline que muestra la tendencia histórica.

use crate::core::read_registers;
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Sparkline},
    Terminal,
};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

/// Runs the interactive dashboard for monitoring a Modbus register.
///
/// # Arguments
/// * `config_path` - Path to the YAML configuration file
/// * `address` - Register address to monitor
/// * `interval_ms` - Sampling interval in milliseconds
///
/// # Returns
/// A Result indicating success or failure. The dashboard exits when 'q' or ESC is pressed.
///
/// # Panics
/// May panic if terminal operations fail.
///
/// Ejecuta el dashboard interactivo para monitorear un registro Modbus.
///
/// # Argumentos
/// * `config_path` - Ruta al archivo de configuración YAML
/// * `address` - Dirección del registro a monitorear
/// * `interval_ms` - Intervalo de muestreo en milisegundos
///
/// # Retorna
/// Un Result que indica éxito o fallo. El dashboard sale cuando se presiona 'q' o ESC.
///
/// # Pánicos
/// Puede entrar en pánico si las operaciones de terminal fallan.
pub async fn run_dashboard(
    config_path: &std::path::Path,
    address: u16,
    interval_ms: u64,
) -> Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let (tx, mut rx) = mpsc::channel::<Vec<u64>>(32);

    let config_path = config_path.to_path_buf();
    let interval = Duration::from_millis(interval_ms);
    tokio::spawn(async move {
        let mut data_history = Vec::new();
        loop {
            match read_registers(&config_path, "holding", address, 1).await {
                Ok(vals) => {
                    if let Some(&val) = vals.first() {
                        data_history.push(val as u64);
                        if data_history.len() > 100 {
                            data_history.remove(0);
                        }
                        if tx.send(data_history.clone()).await.is_err() {
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading register: {}", e);
                }
            }
            time::sleep(interval).await;
        }
    });

    let mut data: Vec<u64> = Vec::new();

    loop {
        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(10)].as_ref())
                .split(f.size());

            let last_val = data.last().unwrap_or(&0);
            let title_block = Block::default()
                .borders(Borders::ALL)
                .title(" Modoc Monitor ");
            let title_text =
                Paragraph::new(format!("Register {} - Latest value: {}", address, last_val))
                    .block(title_block)
                    .style(Style::default().fg(Color::Cyan));
            f.render_widget(title_text, chunks[0]);

            let sparkline = Sparkline::default()
                .data(&data)
                .style(Style::default().fg(Color::Green))
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(" History (last 100 samples) "),
                );
            f.render_widget(sparkline, chunks[1]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }

        if let Ok(new_data) = rx.try_recv() {
            data = new_data;
        } else {
            if rx.is_empty() && rx.is_closed() {
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}