use git2::Repository;

use crate::commit_table::CommitTable;

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};

pub fn start() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let app = CommitTable::new();
    let res = run_app(&mut terminal, app);

    disable_raw_mode();
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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: CommitTable) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => app.next(),
                KeyCode::Up => app.previous(),
                _ => {}
            }
        }
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, commit_table: &mut CommitTable) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(f.size());

    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let normal_style = Style::default().bg(Color::Blue);
    let header_cells = ["Header1", "Header2", "Header3"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells)
        .style(normal_style)
        .height(1)
        .bottom_margin(1);
    let rows = commit_table.table_items.iter().map(|item| {
        let height = item
            .iter()
            .map(|content| content.chars().filter(|c| *c == '\n').count())
            .max()
            .unwrap_or(0)
            + 1;
        let cells = item.iter().map(|c| Cell::from(*c));
        Row::new(cells).height(height as u16).bottom_margin(1)
    });
    let t = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Table"))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);
    f.render_stateful_widget(t, rects[0], &mut commit_table.table_state);
}

fn lib_git_run() {
    println!("Input path to repo");

    let mut path = String::new();
    io::stdin().read_line(&mut path).expect("failed to readline");

    let len = path.trim_end_matches(&['\r', '\n'][..]).len();
    path.truncate(len);
    drop(len);

    let repo = match Repository::open(path) {
        Ok(repo) => repo,
        Err(e) => panic!("failed to open: {}", e),
    };

    let head = repo.head().unwrap();
    let name = head.name().unwrap();
    println!("{}", name);

    println!("Input name of branch");
    
    let mut branch_name = String::new();
    io::stdin().read_line(&mut branch_name).expect("failed to readline");

    let len = branch_name.trim_end_matches(&['\r', '\n'][..]).len();
    branch_name.truncate(len);
    drop(len);

    let mut remote = repo.find_remote("origin").unwrap();
    remote.connect(git2::Direction::Fetch).unwrap();

    println!("{}", remote.default_branch().unwrap().as_str().unwrap());
}