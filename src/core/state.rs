use crate::arxiv::models::Paper;

#[derive(Debug, Clone, PartialEq)]
pub enum View {
    Search,
    Browse,
    Detail,
    Library,
}

pub struct AppState {
    pub current_view: View,
    pub search_query: String,
    pub search_results: Vec<Paper>,
    pub selected_paper: Option<Paper>,
    pub selected_index: usize,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            current_view: View::Search,
            search_query: String::new(),
            search_results: Vec::new(),
            selected_paper: None,
            selected_index: 0,
        }
    }

    pub fn switch_view(&mut self, view: View) {
        self.current_view = view;
    }

    pub fn select_next(&mut self) {
        if !self.search_results.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.search_results.len();
            self.selected_paper = Some(self.search_results[self.selected_index].clone());
        }
    }

    pub fn select_previous(&mut self) {
        if !self.search_results.is_empty() {
            if self.selected_index == 0 {
                self.selected_index = self.search_results.len() - 1;
            } else {
                self.selected_index -= 1;
            }
            self.selected_paper = Some(self.search_results[self.selected_index].clone());
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
