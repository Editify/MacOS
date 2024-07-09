use anyhow::{Context, Result};
use argh::FromArgs;
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::*,
    widgets::Paragraph,
};
use std::{
    io::{self, Stdout},
    time::Duration,
};

// MAIN CLI

#[derive(FromArgs, PartialEq, Debug)]
/// Reach new heights.
pub struct CLI {
    /// terminal version
    #[argh(switch, short = 'T')]
    pub terminal: Option<bool>,

    #[argh(subcommand)]
    pub sub: Option<SubCommands>,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum SubCommands {
    Alias(Alias),
}

// ALIASES

#[derive(FromArgs, PartialEq, Debug)]
/// Alias for Editify.
#[argh(subcommand, name = "alias")]
pub struct Alias {
    #[argh(subcommand)]
    pub sub: AliasSubCommands,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
pub enum AliasSubCommands {
    AliasInstall(AliasInstall),
}

#[derive(FromArgs, PartialEq, Debug)]
/// Installs alias for editify.
#[argh(subcommand, name = "install")]
pub struct AliasInstall {}

impl CLI {
    pub fn from_env() -> Self {
        argh::from_env()
    }
}

pub fn terminal() -> Result<()> {
    let mut terminal = setup_terminal().context("setup failed")?;
    run(&mut terminal).context("app loop failed")?;
    restore_terminal(&mut terminal).context("restore terminal failed")?;
    Ok(())
}

fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    loop {
        terminal.draw(crate::render_app)?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

fn render_app(frame: &mut Frame) {
    let greeting = Paragraph::new("Hello World! (press 'q' to quit)");
    frame.render_widget(greeting, frame.size());
}

fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
