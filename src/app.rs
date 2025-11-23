use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;
use std::time::Duration;

use crate::config::Config;
use crate::core::state::AppState;
use crate::db::client::DbClient;
use crate::embeddings::generator::EmbeddingGenerator;
use crate::ui::app::Ui;

pub struct App {
    config: Config,
    state: AppState,
    db_client: DbClient,
    embedding_gen: EmbeddingGenerator,
    should_quit: bool,
}

impl App {
    pub async fn new(config: Config) -> Result<Self> {
        // Initialize database client
        let db_client = DbClient::new(&config.helixdb)?;

        // Initialize embedding generator
        let embedding_gen = EmbeddingGenerator::new(&config.embeddings)?;

        // Initialize application state
        let state = AppState::new();

        Ok(Self {
            config,
            state,
            db_client,
            embedding_gen,
            should_quit: false,
        })
    }

    pub async fn run(&mut self) -> Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Create UI
        let ui = Ui::new();

        // Main event loop
        let tick_rate = Duration::from_millis(self.config.ui.tick_rate_ms);
        loop {
            // Draw UI
            terminal.draw(|f| ui.render(f, &self.state))?;

            // Handle events
            if event::poll(tick_rate)? {
                if let Event::Key(key) = event::read()? {
                    match key.code {
                        KeyCode::Char('q') => {
                            self.should_quit = true;
                        }
                        _ => {
                            // Handle other key events
                            self.handle_key_event(key.code).await?;
                        }
                    }
                }
            }

            // Check if we should quit
            if self.should_quit {
                break;
            }
        }

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        Ok(())
    }

    async fn handle_key_event(&mut self, _key: KeyCode) -> Result<()> {
        // TODO: Implement key event handling
        Ok(())
    }
}
