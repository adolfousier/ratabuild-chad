mod config;
mod db;
mod tracking;
mod ui;
mod utils;

use ratatui::crossterm::{
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io::stdout;
use ui::app::App;



#[cfg(test)]
mod db_tests {
    include!("tests/db_tests.rs");
}

#[cfg(test)]
mod tracking_tests {
    include!("tests/tracking_tests.rs");
}

#[cfg(test)]
mod ui_tests {
    include!("tests/ui_tests.rs");
}

#[cfg(test)]
mod watcher_tests {
    include!("tests/watcher_tests.rs");
}

#[cfg(test)]
mod utils_tests {
    include!("tests/utils_tests.rs");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize the app
    let mut app = match App::new().await {
        Ok(app) => app,
        Err(e) => {
            eprintln!("App init error: {:?}", e);
            return Err(e);
        }
    };

    // Run the app
    let res = app.run(&mut terminal).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Run error: {:?}", err);
    }

    Ok(())
}
