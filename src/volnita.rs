use git2::Repository;

use crate::{
    traits::display_view::DisplayView, view_components::input_field::InputField,
    views::opened_repo_view::OpenedRepoView,
};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{ErrorKind};
use std::{error::Error, io, path, env};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub fn start() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = run_app(&mut terminal);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}")
    }

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut view = OpenedRepoView::default();
    let args: Vec<String> = env::args().collect();

    if let Ok(items) = lib_git_run(terminal, &args) {
        view.repo_commits.table_items = items;
    }
    else {
        return Err(std::io::Error::new(ErrorKind::InvalidInput, "Invalid path"));
    }

    loop {
        let mut run = false;

        terminal.draw(|f| run = view.display_view(f))?;

        if !run {
            return Ok(());
        }
    }
}

//TODO: add function that gets the repository from the user that uses a loop and returns a repository

fn lib_git_run<B: Backend>(terminal: &mut Terminal<B>, args: &Vec<String>) -> Result<Vec<Vec<String>>, ()> {
    let mut input_field: InputField = InputField::default();
    let repo_path: &str = args.get(0).unwrap(); //change to ind 1

    let repo: Repository;

    if let Ok(arg_repo) = Repository::open(repo_path) {
        repo = arg_repo;
    }
    else {
        let repo_path = input_field
            .input_prompt(terminal, "Input your git repository: ")
            .unwrap();

        let mut path = repo_path.to_owned();

        let len = path.trim_end_matches(&['\r', '\n'][..]).len();
        path.truncate(len);

        path = path.replace('\\', "/");

        repo = match Repository::open(path) {
            Ok(repo) => repo,
            Err(e) => return Err(()),
        };
    }

    let head = repo.head().unwrap();
    //let name = head.name().unwrap();

    let commit = head.peel_to_commit().unwrap();
    let mut commit_history = Vec::new();
    let mut parent = commit.parents().next();

    let commit_item = vec![
        commit.message().unwrap().to_owned(),
        commit.author().name().unwrap().to_owned(),
        commit.id().to_string(),
    ];

    commit_history.push(commit_item);

    for _ in 0..100 {
        match parent {
            Some(p_commit) => {
                let commit_item = vec![
                    p_commit.message().unwrap().to_owned(),
                    p_commit.author().name().unwrap().to_owned(),
                    p_commit.id().to_string(),
                ];

                commit_history.push(commit_item);

                parent = p_commit.parents().next();
            }
            None => break,
        }
    }

    /*let cfg = repo.config().unwrap();
       let mut entries = cfg.entries(None).unwrap();
       while let Some(entry) = entries.next() {
           //let entry = entry.unwrap();
           //println!("{} => {}", entry.name().unwrap(), entry.value().unwrap());
       }
    */

    /*let mut branch_name = input_field.input_prompt(terminal, "Input name of branch").unwrap().to_owned();
    let len = branch_name.trim_end_matches(&['\r', '\n'][..]).len();
    branch_name.truncate(len);
    drop(len);

    let mut remote = repo.find_remote("origin").unwrap();
    remote.connect(git2::Direction::Fetch).unwrap();

    println!("{}", remote.default_branch().unwrap().as_str().unwrap());
    println!("{}", remote.name().unwrap());*/

    Ok(commit_history)
}
