// Main app structure for TUI

use ratatui::{
    Frame, Terminal,
    backend::Backend,
    crossterm::event::{self, Event, KeyCode, KeyModifiers},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, ListState, Padding, Paragraph},
};
use crate::config::settings::{load_config, save_config};
use std::sync::{Arc, Mutex};
use tokio::sync::oneshot;
use crate::tracking::logger::BuildLogger;
use crate::tracking::watcher::BuildWatcher;
use crate::config::Config;
use sqlx::{Row, types::chrono::{DateTime, Utc}};
use std::io;
use std::path::Path;
use walkdir::WalkDir;
use crate::utils::{detect_language_for_path, calculate_dir_size};
use crate::ui::popup::{PopupState, PopupCommand};

pub struct App {
    pub should_quit: bool,
    pub artifacts: Vec<String>,
    pub scanning: bool,
    pub selected: usize,
    pub focused_panel: usize,
    pub logger: BuildLogger,
    pub build_history: Vec<String>,
    pub total_builds: usize,
    pub chart_data: Vec<(String, u64)>,
    pub chart_selected: usize,
    pub watcher: BuildWatcher,
    pub automatic_removal: bool,
    pub config: Config,
    pub popup_state: PopupState,
    pub logs: Arc<Mutex<Vec<String>>>,
    pub pending_action: Option<String>,
    pub pending_failed_paths: Vec<String>,
}

impl App {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let config = load_config();
        let logger = BuildLogger::new(&config.database_url).await?;
        let watcher = BuildWatcher::new();
        let mut app = App {
            should_quit: false,
            artifacts: vec![], // Start empty
            scanning: false,
            selected: 0,
            focused_panel: 0,
            logger,
            build_history: vec![],
            total_builds: 0,
            chart_data: vec![],
            chart_selected: 0,
            watcher,
            automatic_removal: true,
            config,
            popup_state: PopupState::None,
            logs: Arc::new(Mutex::new(vec![])),
            pending_action: None,
            pending_failed_paths: vec![],
        };
        app.load_artifacts().await;
        app.load_history().await;
        Ok(app)
    }

    pub async fn run<B: Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
        terminal.draw(|f| self.draw(f))?;

        self.handle_event().await;

        if self.should_quit {
                break;
            }
        }
        Ok(())
    }

    async fn handle_event(&mut self) {
        if let Ok(Event::Key(key)) = event::read() {
            // Global Shift+D for clear all builds
            if key.code == KeyCode::Char('D') && key.modifiers.contains(KeyModifiers::SHIFT) {
                self.popup_state = PopupState::new_clear_all_confirmation();
                return;
            }
            // Handle popup first
            if let Some(cmd) = self.popup_state.handle_key(&key) {
                match cmd {
                    PopupCommand::OpenInput { title, initial } => {
                        let initial = if title == "Retention Days" {
                            self.config.retention_days.to_string()
                        } else {
                            initial
                        };
                        self.popup_state = PopupState::new_input(title, initial);
                    }
                    PopupCommand::OpenDirBrowse => {
                        self.popup_state = PopupState::new_dir_browse();
                    }
                    PopupCommand::ToggleRemoval => {
                        self.automatic_removal = !self.automatic_removal;
                    }
                    PopupCommand::SetValue { key, value } => {
                        if key == "Retention Days" {
                            if let Ok(days) = value.parse::<u32>() {
                                self.config.retention_days = days;
                            }
                        } else if key == "Scan Path" {
                            self.config.scan_paths = vec![value];
                         } else if key == "Enter sudo password" {
                             if let Some(action) = self.pending_action.take() {
                                 if action == "delete" {
                                     let path = self.artifacts[self.selected].clone();
                                     if self.delete_with_sudo(&path, Some(&value)) {
                                         self.artifacts.remove(self.selected);
                                         if self.selected >= self.artifacts.len() && self.selected > 0 {
                                             self.selected -= 1;
                                         }
                                         // Update DB
                                         let pool = self.logger.pool.clone();
                                         let _ = tokio::spawn(async move {
                                             let _ = sqlx::query("DELETE FROM builds WHERE artifact_path = $1").bind(&path).execute(&pool).await;
                                         });
                                         self.popup_state = PopupState::Info { message: "Artifact deleted with sudo.".to_string() };
                                     } else {
                                         self.popup_state = PopupState::Info { message: "Sudo delete failed.".to_string() };
                                     }
                                 } else if action == "clear_all" {
                                     let failed_paths = self.pending_failed_paths.clone();
                                     self.pending_failed_paths.clear();
                                     let mut all_success = true;
                                     for path in failed_paths {
                                         if !self.delete_with_sudo(&path, Some(&value)) {
                                             all_success = false;
                                         }
                                     }
                                     if all_success {
                                         self.artifacts.clear();
                                         let _ = sqlx::query("DELETE FROM builds").execute(&self.logger.pool).await;
                                         self.load_history().await;
                                         self.popup_state = PopupState::Info { message: "All builds cleared with sudo.".to_string() };
                                     } else {
                                         self.popup_state = PopupState::Info { message: "Some sudo deletes failed.".to_string() };
                                     }
                                 }
                             }
                        }
                        // Save config after changes
                        save_config(&self.config).ok();
                    }
                    PopupCommand::DeleteArtifact => {
                        self.popup_state = PopupState::new_confirm_action("Delete this artifact?".to_string(), "delete".to_string());
                    }
                    PopupCommand::RebuildArtifact => {
                        self.popup_state = PopupState::new_confirm_action("Rebuild this project?".to_string(), "rebuild".to_string());
                    }
                    PopupCommand::ClearAllBuilds => {
                        self.clear_all_builds().await;
                    }
                    PopupCommand::ConfirmAction { action } => {
                         match action.as_str() {
                             "delete" => {
                                 self.popup_state = PopupState::new_progress("Deleting artifact...".to_string());
                                 self.delete_selected().await;
                                 // delete_selected sets the popup_state
                             }
                            "rebuild" => {
                                self.rebuild_selected();
                                self.popup_state = PopupState::new_progress("Rebuilding project...".to_string());
                            }
                            _ => {}
                        }
                    }
                }
            } else if matches!(self.popup_state, PopupState::None) {
                // Main keys only when no popup
                match key.code {
                    KeyCode::Enter => {
                        if self.focused_panel == 0 {
                            self.popup_state = PopupState::new_artifact_actions();
                        } else if self.focused_panel == 3 {
                            self.popup_state = PopupState::new_settings_list();
                        }
                    },
                    KeyCode::Char('q') => self.should_quit = true,
                    KeyCode::Tab => self.focused_panel = (self.focused_panel + 1) % 5,
                    KeyCode::Char('s') => if !self.scanning { self.trigger_scan().await; },
                     KeyCode::Char('d') => self.popup_state = PopupState::new_confirm_action("Delete this artifact?".to_string(), "delete".to_string()),
                    KeyCode::Char('r') => self.rebuild_selected(),
                    KeyCode::Char('h') => self.load_history().await,
                    KeyCode::Char('e') => self.popup_state = PopupState::new_settings_list(),
                     KeyCode::Char('l') => self.popup_state = PopupState::new_logs_popup(Arc::clone(&self.logs)),
                     KeyCode::Up | KeyCode::PageUp => {
                         if self.focused_panel == 0 && self.selected > 0 {
                             self.selected -= 1;
                         } else if self.focused_panel == 2 && self.chart_selected > 0 {
                             self.chart_selected -= 1;
                         }
                     }
                     KeyCode::Down | KeyCode::PageDown => {
                         if self.focused_panel == 0 && self.selected < self.artifacts.len().saturating_sub(1) {
                             self.selected += 1;
                         } else if self.focused_panel == 2 && self.chart_selected < self.chart_data.len().saturating_sub(1) {
                             self.chart_selected += 1;
                         }
                     }
                    _ => {}
                }
            } else {
                // Popup open, only allow quit
                if key.code == KeyCode::Char('q') {
                    self.should_quit = true;
                }
            }
        }
    }

    fn draw(&self, f: &mut Frame) {
        let size = f.area();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(size);

        let title = Paragraph::new("🐀 Ratabuild Chad - Build Artifact Tool")
            .style(
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            )
            .block(Block::default().borders(Borders::ALL));

        f.render_widget(title, chunks[0]);

        self.draw_overview_all_panels(f, chunks[1]);

        self.popup_state.draw(f, size);

        let footer = Paragraph::new("Tab: Focus Panel | s: Scan | h: Load History | ↑↓: Navigate | r: Rebuild | e: Edit Settings | l: Logs | Shift+D: Clear All | q: Quit")
            .style(Style::default().fg(Color::Black).bg(Color::LightGreen));
        f.render_widget(footer, chunks[2]);
    }

    fn draw_overview_all_panels(&self, f: &mut Frame, area: Rect) {
        // Grid layout: 2 rows, 3 columns for 5 panels
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(8), Constraint::Min(8)])
            .split(area);

        let _top_row = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(rows[0]);

        let bottom_row = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)])
            .split(rows[1]);

        // Top row: Artifacts, History, Charts
        self.draw_artifacts_mini(f, _top_row[0], self.focused_panel == 0);
        self.draw_history_mini(f, _top_row[1], self.focused_panel == 1);
        self.draw_charts_mini(f, _top_row[2], self.focused_panel == 2);

        // Bottom row: Settings, Summary
        self.draw_settings_mini(f, bottom_row[0], self.focused_panel == 3);
        self.draw_overview_summary(f, bottom_row[1], self.focused_panel == 4);
    }

    fn draw_artifacts_mini(&self, f: &mut Frame, area: Rect, focused: bool) {
        let border_style = if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let (start, take_count) = if focused {
            (0, self.artifacts.len())
        } else {
            (if self.selected > 2 { self.selected - 2 } else { 0 }, 3)
        };
        let items: Vec<ListItem> = self
            .artifacts
            .iter()
            .skip(start)
            .take(take_count)
            .enumerate()
            .map(|(i, a)| {
                let color = if a.contains("target") {
                    Color::Green
                } else if a.contains("node_modules") {
                    Color::Blue
                } else if a.contains("__pycache__") {
                    Color::Yellow
                } else if a.contains("build") {
                    Color::Red
                } else {
                    Color::White
                };
                let style = if focused && i + start == self.selected {
                    Style::default().bg(Color::Blue).fg(Color::Black)
                } else {
                    Style::default().fg(color)
                };
                ListItem::new(Span::styled(format!("📁 {}", a), style))
            })
            .collect();
        let mut state = ListState::default();
        state.select(Some(self.selected));
        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title("📦 Artifacts")
                .padding(Padding::new(1,1,1,0)),
        );
        f.render_stateful_widget(list, area, &mut state);
    }

    fn draw_history_mini(&self, f: &mut Frame, area: Rect, focused: bool) {
        let border_style = if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let history_text = self.build_history.join("\n");
        let para = Paragraph::new(history_text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title("📜 History")
                .padding(Padding::new(1,1,1,0)),
        );
        f.render_widget(para, area);
    }

    fn draw_charts_mini(&self, f: &mut Frame, area: Rect, focused: bool) {
        let border_style = if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let items: Vec<ListItem> = if self.chart_data.is_empty() {
            vec![ListItem::new("No data")]
        } else {
            let max_size = self.chart_data.iter().map(|(_, s)| *s).max().unwrap_or(1);
            let colors = [Color::Red, Color::Green, Color::Blue, Color::Yellow, Color::Magenta, Color::Cyan, Color::White];
            let scan_path = self.config.scan_paths.first().map(|s| s.as_str()).unwrap_or("");
            self.chart_data.iter().enumerate().map(|(i, (name, size))| {
                let bar_len = if max_size > 0 { (size * 25 / max_size) as usize } else { 0 };
                let bar = "█".repeat(bar_len);
                let size_mb = size / 1_000_000;
                let color = colors[i % colors.len()];
                let style = if focused && i == self.chart_selected {
                    Style::default().bg(Color::Blue).fg(Color::Black)
                } else {
                    Style::default().fg(color)
                };
                let relative_name = if let Some(stripped) = name.strip_prefix(&format!("{}/", scan_path)) {
                    stripped
                } else {
                    name
                };
                let short_name = if relative_name.len() > 20 { format!("{}...", &relative_name[..17]) } else { relative_name.to_string() };
                ListItem::new(Span::styled(format!("{} {} {} MB", short_name, bar, size_mb), style))
            }).collect()
        };
        let mut state = ListState::default();
        state.select(Some(self.chart_selected));
        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title("📊 Charts")
                .padding(Padding::new(1,1,1,0)),
        );
        f.render_stateful_widget(list, area, &mut state);
    }

    fn draw_settings_mini(&self, f: &mut Frame, area: Rect, focused: bool) {
        let border_style = if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let masked_db = Self::mask_db_url(&self.config.database_url);
        let removal_status = if self.automatic_removal { "Enabled" } else { "Disabled" };
        let text = format!(
            "DB: {}\nPaths: {}\nRetention Days: {}\nAutomatic Removal: {}",
            masked_db,
            self.config.scan_paths.join(","),
            self.config.retention_days,
            removal_status
        );
        let para = Paragraph::new(text).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .padding(Padding::new(1,1,1,0))
                .title("⚙️ Settings"),
        );
        f.render_widget(para, area);
    }



    fn draw_overview_summary(&self, f: &mut Frame, area: Rect, focused: bool) {
        let border_style = if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        };
        let summary = format!(
            "🏗️ Total Builds: {}\n📦 Artifacts: {}\n🔍 Scans: Active\n⚡ Watcher: Running",
            self.total_builds,
            self.artifacts.len()
        );
        let para = Paragraph::new(summary).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title("🏠 Summary")
                .padding(Padding::new(1,1,1,0)),
        );
        f.render_widget(para, area);
    }



    async fn trigger_scan(&mut self) {
        self.scanning = true;
        self.popup_state = PopupState::Scanning { logs: Arc::clone(&self.logs) };
        let scan_paths = if self.config.scan_paths.is_empty() {
            vec![".".to_string()]
        } else {
            self.config.scan_paths.clone()
        };
        let logs_clone = Arc::clone(&self.logs);
        let artifacts_clone = Arc::new(Mutex::new(vec![]));
        let _artifacts_clone2 = Arc::clone(&artifacts_clone);
        let logger_clone = self.logger.clone();
        let mut watcher_clone = self.watcher.clone();
        let _config_clone = self.config.clone();
        let (tx, rx) = oneshot::channel::<Vec<String>>();
        tokio::spawn(async move {
            {
                let mut logs = logs_clone.lock().unwrap();
                logs.push("Starting scan...".to_string());
            }
            let common_dirs = [
                "target",
                "build",
                ".build",
                "node_modules",
                "__pycache__",
                "dist",
                "out",
                "vendor",
                "cmake-build-debug",
                "cmake-build-release",
                "Debug",
                "Release",
            ];
            let mut total_count = 0;
            for scan_path in scan_paths {
                {
                    let mut logs = logs_clone.lock().unwrap();
                    logs.push(format!("Scanning path: {}", scan_path));
                }
                let mut count = 0;
                for entry in WalkDir::new(&scan_path)
                    .max_depth(3)
                    .into_iter()
                    .filter_map(|e| e.ok())
                {
                    if entry.file_type().is_dir() {
                        let name = entry.file_name().to_string_lossy();
                        if common_dirs.contains(&name.as_ref()) {
                            let path_str = entry.path().display().to_string();
                            let project_path = entry.path().parent().unwrap_or(Path::new(".")).display().to_string();
                            let language = detect_language_for_path(&project_path);
                            let size = calculate_dir_size(&path_str);
                            {
                                let mut artifacts = artifacts_clone.lock().unwrap();
                                artifacts.push(path_str.clone());
                            }
                            count += 1;
                            // Log to DB
                            let _ = logger_clone
                                .log_build(&project_path, &language, &path_str, size)
                                .await;
                            // Start watching
                            let _ = watcher_clone.watch(&path_str);
                        }
                    }
                }
                total_count += count;
                {
                    let mut logs = logs_clone.lock().unwrap();
                    logs.push(format!("Scan complete for {}. Found {} artifacts.", scan_path, count));
                }
            }
            let artifacts = artifacts_clone.lock().unwrap().clone();
            let _ = tx.send(artifacts);
            {
                let mut logs = logs_clone.lock().unwrap();
                logs.push(format!("Total scan complete. Found {} artifacts.", total_count));
            }
        });
        let artifacts = rx.await.unwrap_or_default();
        self.artifacts = artifacts;
        self.scanning = false;
        self.popup_state = PopupState::None;
        // Refresh history after scan
        self.load_history().await;
    }

    async fn delete_selected(&mut self) {
        if self.artifacts.is_empty() {
            return;
        }
        let path = self.artifacts[self.selected].clone();
        // Check for unusual files (e.g., bundle or many binaries)
        if self.has_unusual_files(&path) {
            return;
        }
        // Try sudo -n first
        if self.delete_with_sudo(&path, None) {
            self.artifacts.remove(self.selected);
            if self.selected >= self.artifacts.len() && self.selected > 0 {
                self.selected -= 1;
            }
            // Update DB
            let _ = sqlx::query("DELETE FROM builds WHERE artifact_path = $1").bind(&path).execute(&self.logger.pool).await;
            self.popup_state = PopupState::Info { message: "Artifact deleted.".to_string() };
        } else {
            // Prompt for password
            self.pending_action = Some("delete".to_string());
            self.popup_state = PopupState::new_input("Enter sudo password".to_string(), "".to_string());
        }
    }

    async fn load_artifacts(&mut self) {
        // Query DB for recent artifact paths
        match sqlx::query("SELECT artifact_path FROM builds GROUP BY artifact_path ORDER BY MAX(build_time) DESC LIMIT 50")
            .fetch_all(&self.logger.pool)
            .await
        {
            Ok(rows) => {
                for row in rows {
                    let path: String = row.get(0);
                    self.artifacts.push(path.clone());
                    // Start watching
                    let _ = self.watcher.watch(&path);
                }
            }
            Err(_) => {
                // Ignore errors, start empty
            }
        }
    }

    async fn load_history(&mut self) {
        // Query DB for build history
        match sqlx::query("SELECT project_path, language, build_time FROM builds ORDER BY build_time DESC LIMIT 10")
            .fetch_all(&self.logger.pool)
            .await
        {
            Ok(rows) => {
                self.build_history = rows
                    .into_iter()
                    .map(|row| {
                        let project: String = row.get(0);
                        let language: String = row.get(1);
                        let time: DateTime<Utc> = row.get(2);
                        format!("{} - {} - {}", project, language, time.format("%Y-%m-%d %H:%M"))
                    })
                    .collect();
            }
            Err(_) => {
                self.build_history = vec!["Failed to load history".to_string()];
            }
        }
        match sqlx::query("SELECT COUNT(*) FROM builds")
            .fetch_one(&self.logger.pool)
            .await
        {
            Ok(row) => {
                self.total_builds = row.get::<i64, _>(0) as usize;
            }
            Err(_) => {
                self.total_builds = 0;
            }
        }
        match sqlx::query("SELECT artifact_path, MAX(size_bytes) as size FROM builds GROUP BY artifact_path ORDER BY size DESC LIMIT 10")
            .fetch_all(&self.logger.pool)
            .await
        {
            Ok(rows) => {
                self.chart_data = rows
                    .into_iter()
                    .map(|row| (row.get(0), row.get::<i64, _>(1) as u64))
                    .collect();
            }
            Err(_) => {
                self.chart_data = vec![];
            }
        }
    }

    fn mask_db_url(url: &str) -> String {
        if let Some(at_pos) = url.find('@') {
            let before = &url[..at_pos];
            if before.contains(':') {
                format!("***:***@{}", &url[at_pos + 1..])
            } else {
                format!("***@{}", &url[at_pos + 1..])
            }
        } else {
            "configured".to_string()
        }
    }

    fn has_unusual_files(&self, path: &str) -> bool {
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_lowercase();
                if name.contains("bundle") || name.ends_with(".exe") || name.ends_with(".bin") {
                    return true;
                }
            }
        }
        false
    }

    fn rebuild_selected(&mut self) {
        if self.artifacts.is_empty() {
            return;
        }
        let artifact_path = &self.artifacts[self.selected];
        let project_root = std::path::Path::new(artifact_path)
            .parent()
            .unwrap_or(std::path::Path::new("."));
        // Detect build system
        if project_root.join("Cargo.toml").exists() {
            std::process::Command::new("sh")
                .arg("-c")
                .arg("cargo build")
                .current_dir(project_root)
                .spawn()
                .ok(); // Fire and forget
        } else if project_root.join("package.json").exists() {
            std::process::Command::new("sh")
                .arg("-c")
                .arg("npm run build")
                .current_dir(project_root)
                .spawn()
                .ok();
        }
        // Add more as needed
    }

    async fn clear_all_builds(&mut self) {
        let mut failed_paths = vec![];
        for path in self.artifacts.clone() {
            if !self.delete_with_sudo(&path, None) {
                failed_paths.push(path);
            }
        }
        if failed_paths.is_empty() {
            self.artifacts.clear();
            let _ = sqlx::query("DELETE FROM builds").execute(&self.logger.pool).await;
            self.load_history().await;
            self.popup_state = PopupState::Info { message: "All builds cleared.".to_string() };
        } else {
            self.pending_failed_paths = failed_paths;
            self.pending_action = Some("clear_all".to_string());
            self.popup_state = PopupState::new_input("Enter sudo password".to_string(), "".to_string());
        }
    }

    fn delete_with_sudo(&self, path: &str, password: Option<&str>) -> bool {
        use std::process::Command;
        let mut cmd = Command::new("sudo");
        if password.is_some() {
            cmd.arg("-S");
        } else {
            cmd.arg("-n");
        }
        cmd.arg("rm").arg("-rf").arg(path);
        if let Some(pwd) = password {
            cmd.stdin(std::process::Stdio::piped());
            let mut child = cmd.spawn().unwrap();
            if let Some(mut stdin) = child.stdin.take() {
                use std::io::Write;
                let _ = stdin.write_all(format!("{}\n", pwd).as_bytes());
            }
            child.wait().unwrap().success()
        } else {
            cmd.status().unwrap().success()
        }
    }
}
