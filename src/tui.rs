use super::server;
use crossterm::{execute, terminal::*};
use ratatui::prelude::*;
use std::io::{self, stdout, Stdout};

/// A type alias for the terminal type used in this application
pub type Backend = Terminal<CrosstermBackend<Stdout>>;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::widgets::{block::*, *};

#[derive(Debug, Default)]
pub struct Tui {
    server: server::Server,
    outlog: Vec<String>,
    errlog: Vec<String>,
    tasklist: ListState,
    outputlist: ListState,
    errorlist: ListState,
    exit: bool,
}
impl Tui {
    pub fn new(server: server::Server) -> Self {
        Self {
            server,
            tasklist: ListState::default().with_selected(Some(0)),
            ..Default::default()
        }
    }
    /// runs the application's main loop until the user quits
    pub fn run(&mut self) -> io::Result<()> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        terminal.clear()?;
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(20),
                Constraint::Length(1),
                Constraint::Percentage(80),
            ])
            .split(frame.size());
        //render the list
        let list = List::new(
            self.server
                .cfg
                .tasks
                .iter()
                .map(|t| ListItem::new(t.name.clone())),
        )
        .block(Block::default().title(Title::from("Tasks".blue()).alignment(Alignment::Center)))
        .highlight_style(Style::default().bg(Color::LightBlue).fg(Color::Black))
        .highlight_symbol(">>");
        frame.render_stateful_widget(list, layout[0], &mut self.tasklist);
        //render the split line
        let split_line = Block::default().borders(Borders::LEFT);
        //render the logs
        frame.render_widget(split_line, layout[1]);
        let log_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(layout[2]);
        //render the output log
        let outlog = List::new(
            self.outlog
                .iter()
                .skip(
                    self.outlog
                        .len()
                        .saturating_sub((log_layout[0].height - 1) as usize),
                )
                .map(|o| ListItem::new(o.clone())),
        )
        .block(Block::default().title(Title::from("Output".green()).alignment(Alignment::Center)));
        frame.render_stateful_widget(outlog, log_layout[0], &mut self.outputlist);
        //render the error log
        let errlog = List::new(
            self.errlog
                .iter()
                .skip(
                    self.errlog
                        .len()
                        .saturating_sub((log_layout[1].height - 1) as usize),
                )
                .map(|e| ListItem::new(e.clone())),
        )
        .block(Block::default().title(Title::from("Error".red()).alignment(Alignment::Center)));
        frame.render_stateful_widget(errlog, log_layout[1], &mut self.errorlist);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Up => {
                if let Some(index) = self.tasklist.selected_mut().as_mut() {
                    if *index > 0 {
                        *index -= 1;
                    }
                }
            }
            KeyCode::Down => {
                if let Some(index) = self.tasklist.selected_mut().as_mut() {
                    if *index < self.server.cfg.tasks.len() - 1 {
                        *index += 1;
                    }
                }
            }
            KeyCode::Enter => {
                if let Some(index) = self.tasklist.selected().as_ref() {
                    let (outputs, _) = self.server.exec_by_index(*index);
                    let (out, err): (Vec<_>, Vec<_>) = outputs
                        .iter()
                        .map(|o| {
                            (
                                o.stdout.split(|&b| b == b'\n').collect::<Vec<_>>(),
                                o.stderr.split(|&b| b == b'\n').collect::<Vec<_>>(),
                            )
                        })
                        .unzip();
                    self.outlog.extend(
                        out.concat()
                            .into_iter()
                            .map(|s| String::from_utf8_lossy(s).to_string()),
                    );
                    self.errlog.extend(
                        err.into_iter()
                            .filter(|e| !e.is_empty())
                            .flatten()
                            .map(|s| String::from_utf8_lossy(s).to_string()), // flatten the iterator of iterators
                    );
                }
            }
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}
