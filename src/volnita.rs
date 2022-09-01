use git2::Repository;

use crate::{traits::display_view::DisplayView, views::opened_repo_view::OpenedRepoView};

use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend}, Terminal,
};

pub fn start() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut view = OpenedRepoView::new();
    view.repo_commits.table_items = lib_git_run(&mut terminal);
    let res = run_app(&mut terminal, view);

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

fn run_app<B: Backend, D: DisplayView>(terminal: &mut Terminal<B>, mut view: D) -> io::Result<()> {
    loop {
        terminal.draw(|f| view.display_view(f))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => return Ok(()),
                KeyCode::Down => view.arrow_down(),
                KeyCode::Up => view.arrow_up(),
                _ => {}
            }
        }
    }
}

fn lib_git_run<B: Backend>(terminal: &mut Terminal<B>) -> Vec<Vec<String>> {
    println!("Input path to repo"); //TODO: Do this through TUI

    let mut path = String::new();
    //io::stdin().read_line(&mut path).expect("failed to readline"); //Can't do this, this breaks with TUI

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

    let commit = head.peel_to_commit().unwrap();
    println!("{}", commit.message().unwrap());
    println!("parents: {}", commit.parent_count());
    
    let mut commit_history = Vec::new();

    for i in 1..=commit.parent_count() {
        let cur_commit = commit.parent(i).unwrap();

        let commit_item = vec![
            cur_commit.message().unwrap().to_owned(),
            cur_commit.id().to_string(),
            cur_commit.author().name().unwrap().to_owned(),
        ];

        commit_history.push(commit_item);
    }

    for commit_item in commit.parents() {
        println!("{}", commit_item.message().unwrap());
    }

    let cfg = repo.config().unwrap();
    let mut entries = cfg.entries(None).unwrap();
    while let Some(entry) = entries.next() {
        //let entry = entry.unwrap();
        //println!("{} => {}", entry.name().unwrap(), entry.value().unwrap());
    }

    println!("Input name of branch");
    
    let mut branch_name = String::new();
    io::stdin().read_line(&mut branch_name).expect("failed to readline");

    let len = branch_name.trim_end_matches(&['\r', '\n'][..]).len();
    branch_name.truncate(len);
    drop(len);

    let mut remote = repo.find_remote("origin").unwrap();
    remote.connect(git2::Direction::Fetch).unwrap();

    println!("{}", remote.default_branch().unwrap().as_str().unwrap());
    println!("{}", remote.name().unwrap());

    commit_history
}