use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::core::state::{AppState, View};

pub struct Ui;

impl Ui {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, f: &mut Frame, state: &AppState) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Min(0),     // Main content
                Constraint::Length(3),  // Footer
            ])
            .split(f.size());

        // Render header
        let header = Paragraph::new("arXiv TUI - Semantic Paper Search")
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Render main content based on current view
        match state.current_view {
            View::Search => self.render_search_view(f, state, chunks[1]),
            View::Browse => self.render_browse_view(f, state, chunks[1]),
            View::Detail => self.render_detail_view(f, state, chunks[1]),
            View::Library => self.render_library_view(f, state, chunks[1]),
        }

        // Render footer
        let footer = Paragraph::new("Press 'q' to quit | '/' to search | 'h' for help")
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(footer, chunks[2]);
    }

    fn render_search_view(&self, f: &mut Frame, _state: &AppState, area: ratatui::layout::Rect) {
        let content = Paragraph::new("Search View - Coming Soon!")
            .block(Block::default().title("Search").borders(Borders::ALL));
        f.render_widget(content, area);
    }

    fn render_browse_view(&self, f: &mut Frame, _state: &AppState, area: ratatui::layout::Rect) {
        let content = Paragraph::new("Browse View - Coming Soon!")
            .block(Block::default().title("Browse").borders(Borders::ALL));
        f.render_widget(content, area);
    }

    fn render_detail_view(&self, f: &mut Frame, _state: &AppState, area: ratatui::layout::Rect) {
        let content = Paragraph::new("Detail View - Coming Soon!")
            .block(Block::default().title("Paper Detail").borders(Borders::ALL));
        f.render_widget(content, area);
    }

    fn render_library_view(&self, f: &mut Frame, _state: &AppState, area: ratatui::layout::Rect) {
        let content = Paragraph::new("Library View - Coming Soon!")
            .block(Block::default().title("Library").borders(Borders::ALL));
        f.render_widget(content, area);
    }
}

impl Default for Ui {
    fn default() -> Self {
        Self::new()
    }
}
