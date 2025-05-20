use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::{error::Error, io, time::{Duration, Instant}};

mod battery;
mod event_state;
mod game_state;
mod game_variables;
mod lunar_phase;
mod operations;
mod generator;
mod solar;

use game_state::GameState;
use game_variables::GameVariables;

struct App {
    last_tick: Instant,
    game_state: GameState,
    game_vars: GameVariables,
}

impl App {
    fn new() -> App {
        App {
            last_tick: Instant::now(),
            game_state: GameState::new(),
            game_vars: GameVariables::default(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        let tick_rate = Duration::from_secs(1);
        if app.last_tick.elapsed() >= tick_rate {
            app.game_state.tick(&app.game_vars);
            app.last_tick += tick_rate;
        }
        terminal.draw(|f| ui(f, app))?;

        if crossterm::event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if KeyCode::Char('q') == key.code {
                    return Ok(());
                }
            }
        }
        
    }
}

fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(f.size());

    let mission_time = app.game_state.mission_time_seconds;
    let timer_text = format!("Mission Time: {}s", mission_time);
    let timer_paragraph = Paragraph::new(timer_text)
        .block(Block::default().title("Mission Timer").borders(Borders::ALL));
    f.render_widget(timer_paragraph, chunks[0]);
}

