// src/tui.rs

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io;

use crate::garden::MainGameState;

pub fn draw_ui(game_state: &mut MainGameState) -> io::Result<()> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app and run it
    let res = run_app(&mut terminal, game_state);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    game_state: &mut MainGameState,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, game_state))?;

        if let Event::Key(key) = event::read()? {
            if let KeyCode::Char('q') = key.code {
                return Ok(());
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, game_state: &MainGameState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
        .split(f.size());

    let garden_view = if let Some(plot) = game_state.plots.get(&(0, 0)) {
        let mut garden_str = String::new();
        for row in &plot.grid.tiles {
            for tile in row {
                let symbol = match &tile.plant {
                    Some(plant) => match plant.life_cycle_stage {
                        crate::plant::LifeCycleStage::Seed => 's',
                        crate::plant::LifeCycleStage::Sprout => 'p',
                        crate::plant::LifeCycleStage::Growing => 'P',
                        crate::plant::LifeCycleStage::Mature => 'M',
                        crate::plant::LifeCycleStage::Fruiting => 'F',
                        crate::plant::LifeCycleStage::Withering => 'x',
                    },
                    None => '.',
                };
                garden_str.push(symbol);
                garden_str.push(' ');
            }
            garden_str.push('\n');
        }
        Paragraph::new(garden_str)
    } else {
        Paragraph::new("No plot found.")
    };

    let garden_block = Block::default().title("Garden View").borders(Borders::ALL);
    f.render_widget(garden_view.block(garden_block), chunks[0]);

    let status_text = format!(
        "Tick: {} | Money: ${:.2}",
        game_state.tick_counter, game_state.wallet
    );
    let status_paragraph = Paragraph::new(status_text);

    let status_block = Block::default()
        .title("Command/Status Bar")
        .borders(Borders::ALL);
    f.render_widget(status_paragraph.block(status_block), chunks[1]);
}
