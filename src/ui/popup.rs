// Popup functionality for settings editing

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Padding, Paragraph},
    crossterm::event::KeyCode,
};

pub enum PopupCommand {
    OpenInput { title: String, initial: String },
    OpenDirBrowse,
    ToggleRemoval,
    SetValue { key: String, value: String },
    DeleteArtifact,
    RebuildArtifact,
    ClearAllBuilds,
    ConfirmAction { action: String },
}

pub enum PopupState {
    None,
    SettingsList { selected: usize },
    Input { title: String, input: String },
    DirBrowse { path: String, items: Vec<String>, selected: usize },
    Logs { logs: std::sync::Arc<std::sync::Mutex<Vec<String>>> },
    Scanning { logs: std::sync::Arc<std::sync::Mutex<Vec<String>>> },
    ArtifactActions { selected: usize },
    ClearAllConfirmation,
    ConfirmAction { message: String, action: String },
    Progress { message: String },
    Info { message: String },
}

impl PopupState {
    pub fn new_settings_list() -> Self {
        PopupState::SettingsList { selected: 0 }
    }

    pub fn new_input(title: String, initial: String) -> Self {
        PopupState::Input { title, input: initial }
    }

    pub fn new_dir_browse() -> Self {
        let path = "/".to_string();
        let items = get_dir_items(&path);
        PopupState::DirBrowse { path, items, selected: 0 }
    }

    pub fn new_logs_popup(logs: std::sync::Arc<std::sync::Mutex<Vec<String>>>) -> Self {
        PopupState::Logs { logs }
    }

    pub fn new_artifact_actions() -> Self {
        PopupState::ArtifactActions { selected: 0 }
    }

    pub fn new_clear_all_confirmation() -> Self {
        PopupState::ClearAllConfirmation
    }

    pub fn new_confirm_action(message: String, action: String) -> Self {
        PopupState::ConfirmAction { message, action }
    }

    pub fn new_progress(message: String) -> Self {
        PopupState::Progress { message }
    }
}

impl PopupState {
    pub fn draw(&self, f: &mut Frame, area: Rect) {
        match self {
            PopupState::SettingsList { selected } => {
                let popup_area = centered_rect(25, 25, area);
                f.render_widget(Clear, popup_area);
                let options = ["Retention Days", "Scan Path", "Automatic Removal"];
                let mut items = Vec::new();
                for (i, &opt) in options.iter().enumerate() {
                    let style = if i == *selected {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    items.push(ListItem::new(Span::styled(opt, style)));
                }
                let list = List::new(items)
                    .block(Block::default().title("Settings (↑↓ Enter Esc)").borders(Borders::ALL));
                f.render_widget(list, popup_area);
            }
            PopupState::Input { title, input } => {
                let popup_area = centered_rect(50, 10, area);
                f.render_widget(Clear, popup_area);
                let display_input = if title == "Enter sudo password" {
                    "*".repeat(input.len())
                } else {
                    input.clone()
                };
                let text = format!("{}: {}", title, display_input);
                let para = Paragraph::new(text)
                    .block(Block::default().title("Edit (Enter: Apply, Esc: Cancel)").borders(Borders::ALL));
                f.render_widget(para, popup_area);
                // Cursor not implemented simply
            }
            PopupState::DirBrowse { path, items, selected } => {
                let popup_area = centered_rect(50, 50, area);
                f.render_widget(Clear, popup_area);
                let list_items: Vec<ListItem> = items
                    .iter()
                    .map(|i| ListItem::new(i.as_str()))
                    .collect();
                let list = List::new(list_items)
                    .block(Block::default().title(format!("Browse: {} (↑↓ Nav, Enter: Enter, s: Select, Space: Select Current, Esc: Cancel)", path)).borders(Borders::ALL))
                    .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));
                let mut state = ListState::default();
                state.select(Some(*selected));
                f.render_stateful_widget(list, popup_area, &mut state);
            }
            PopupState::Logs { logs } => {
                let popup_area = centered_rect(60, 40, area);
                f.render_widget(Clear, popup_area);
                let logs_guard = logs.lock().unwrap();
                let logs_text = logs_guard.iter().rev().take(20).cloned().collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n");
                let para = Paragraph::new(logs_text).block(
                    Block::default()
                        .title("📝 Logs")
                        .borders(Borders::ALL)
                        .padding(Padding::new(1, 1, 1, 0)),
                );
                f.render_widget(para, popup_area);
            }
            PopupState::Scanning { logs } => {
                let popup_area = centered_rect(60, 40, area);
                f.render_widget(Clear, popup_area);
                let logs_guard = logs.lock().unwrap();
                let logs_text = logs_guard.iter().rev().take(20).cloned().collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n");
                let para = Paragraph::new(logs_text).block(
                    Block::default()
                        .title("🔍 Scanning...")
                        .borders(Borders::ALL)
                        .padding(Padding::new(1, 1, 1, 0)),
                );
                f.render_widget(para, popup_area);
            }
            PopupState::ArtifactActions { selected } => {
                let popup_area = centered_rect(25, 25, area);
                f.render_widget(Clear, popup_area);
                let options = ["Delete", "Rebuild"];
                let mut items = Vec::new();
                for (i, &opt) in options.iter().enumerate() {
                    let style = if i == *selected {
                        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                    };
                    items.push(ListItem::new(Span::styled(opt, style)));
                }
                let list = List::new(items)
                    .block(Block::default().title("Artifact Actions (↑↓ Enter Esc)").borders(Borders::ALL));
                f.render_widget(list, popup_area);
            }
            PopupState::ClearAllConfirmation => {
                let popup_area = centered_rect(50, 20, area);
                f.render_widget(Clear, popup_area);
                let text = "Are you sure you want to clear all builds?\nThis will delete all artifacts from the filesystem.\n(y: Confirm, n: Cancel)";
                let para = Paragraph::new(text)
                    .block(Block::default().title("⚠️ Clear All Builds").borders(Borders::ALL));
                f.render_widget(para, popup_area);
            }
            PopupState::ConfirmAction { message, .. } => {
                let popup_area = centered_rect(50, 15, area);
                f.render_widget(Clear, popup_area);
                let text = format!("{}\n\nPress Enter to confirm, Esc to cancel.", message);
                let para = Paragraph::new(text)
                    .block(Block::default().title("Confirm Action").borders(Borders::ALL));
                f.render_widget(para, popup_area);
            }
            PopupState::Progress { message } => {
                let popup_area = centered_rect(50, 10, area);
                f.render_widget(Clear, popup_area);
                let text = format!("{}\n\nPress Esc to close.", message);
                let para = Paragraph::new(text)
                    .block(Block::default().title("Progress").borders(Borders::ALL));
                f.render_widget(para, popup_area);
            }
            PopupState::Info { message } => {
                let popup_area = centered_rect(50, 10, area);
                f.render_widget(Clear, popup_area);
                let para = Paragraph::new(message.as_str())
                    .block(Block::default().title("Info").borders(Borders::ALL));
                f.render_widget(para, popup_area);
            }
            PopupState::None => {}
        }
    }

    pub fn handle_key(&mut self, key: &ratatui::crossterm::event::KeyEvent) -> Option<PopupCommand> {
        match self {
            PopupState::SettingsList { selected } => match key.code {
                KeyCode::Up => {
                    if *selected > 0 {
                        *selected -= 1;
                    } else {
                        *selected = 2; // Wrap to last
                    }
                }
                KeyCode::Down => {
                    if *selected < 2 {
                        *selected += 1;
                    } else {
                        *selected = 0; // Wrap to first
                    }
                }
                KeyCode::Enter => {
                    let cmd = match *selected {
                        0 => Some(PopupCommand::OpenInput { title: "Retention Days".to_string(), initial: "".to_string() }), // will set in app
                        1 => Some(PopupCommand::OpenDirBrowse),
                        2 => Some(PopupCommand::ToggleRemoval),
                        _ => None,
                    };
                    if cmd.is_some() {
                        *self = PopupState::None;
                    }
                    return cmd;
                }
                KeyCode::Esc => {
                    *self = PopupState::None;
                }
                _ => {}
            },
            PopupState::Input { title, input } => match key.code {
                KeyCode::Char(c) => {
                    input.push(c);
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Enter => {
                    let value = input.clone();
                    let key = title.clone();
                    *self = PopupState::None;
                    return Some(PopupCommand::SetValue { key, value });
                }
                KeyCode::Esc => {
                    *self = PopupState::None;
                }
                _ => {}
            },
            PopupState::Logs { .. } => match key.code {
                KeyCode::Esc => {
                    *self = PopupState::None;
                }
                _ => {}
            },
            PopupState::Scanning { .. } => {
                // No interaction during scanning
            }
            PopupState::ArtifactActions { selected } => match key.code {
                KeyCode::Up => {
                    if *selected > 0 {
                        *selected -= 1;
                    } else {
                        *selected = 1; // Wrap to last
                    }
                }
                KeyCode::Down => {
                    if *selected < 1 {
                        *selected += 1;
                    } else {
                        *selected = 0; // Wrap to first
                    }
                }
                KeyCode::Enter => {
                    let cmd = match *selected {
                        0 => Some(PopupCommand::DeleteArtifact),
                        1 => Some(PopupCommand::RebuildArtifact),
                        _ => None,
                    };
                    if cmd.is_some() {
                        *self = PopupState::None;
                    }
                    return cmd;
                }
                KeyCode::Esc => {
                    *self = PopupState::None;
                }
                _ => {}
            },
            PopupState::ClearAllConfirmation => match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    *self = PopupState::None;
                    return Some(PopupCommand::ClearAllBuilds);
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    *self = PopupState::None;
                }
                _ => {}
            },
            PopupState::ConfirmAction { action, .. } => {
                let action = action.clone();
                match key.code {
                    KeyCode::Enter => {
                        *self = PopupState::None;
                        return Some(PopupCommand::ConfirmAction { action });
                    }
                    KeyCode::Esc => {
                        *self = PopupState::None;
                    }
                    _ => {}
                }
            }
            PopupState::Progress { .. } => match key.code {
                KeyCode::Esc => {
                    *self = PopupState::None;
                }
                _ => {}
            },
            PopupState::Info { .. } => match key.code {
                KeyCode::Esc => {
                    *self = PopupState::None;
                }
                _ => {}
            },
            PopupState::DirBrowse { path, items, selected } => match key.code {
                KeyCode::Up => {
                    if *selected > 0 {
                        *selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if *selected < items.len().saturating_sub(1) {
                        *selected += 1;
                    }
                }
                KeyCode::Enter => {
                    if *selected < items.len() {
                        let item = &items[*selected];
                        if item == ".." {
                            // Go up
                            if let Some(parent) = std::path::Path::new(path).parent() {
                                *path = parent.display().to_string();
                                *items = get_dir_items(path);
                                *selected = 0;
                            }
                        } else {
                            // Enter dir
                            let new_path = std::path::Path::new(path).join(item);
                            if new_path.is_dir() {
                                *path = new_path.display().to_string();
                                *items = get_dir_items(path);
                                *selected = 0;
                            }
                        }
                    }
                }
                KeyCode::Char('s') => {
                    if *selected < items.len() {
                        let item = &items[*selected];
                        let selected_path = if item == ".." {
                            if let Some(parent) = std::path::Path::new(path).parent() {
                                parent.display().to_string()
                            } else {
                                path.clone()
                            }
                        } else {
                            std::path::Path::new(path).join(item).display().to_string()
                        };
                        *self = PopupState::None;
                        return Some(PopupCommand::SetValue { key: "Scan Path".to_string(), value: selected_path });
                    }
                }
                KeyCode::Char(' ') => {
                    // Select current directory
                    let current_path = path.clone();
                    *self = PopupState::None;
                    return Some(PopupCommand::SetValue { key: "Scan Path".to_string(), value: current_path });
                }
                KeyCode::Esc => {
                    *self = PopupState::None;
                }
                _ => {}
            },
            PopupState::None => {}
        }
        None
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

fn get_dir_items(path: &str) -> Vec<String> {
    let mut items = vec!["..".to_string()];
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    items.push(entry.file_name().to_string_lossy().to_string());
                }
            }
        }
    }
    items
}