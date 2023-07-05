use git2::{Commit, ErrorClass, Repository};

use crate::{
    config::Config,
    traits::display_view::DisplayView,
    views::{opened_repo_view::OpenedRepoView, start_view::StartView},
};

#[cfg(windows)]
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

#[cfg(not(windows))]
use crossterm::{
    event::{
        DisableMouseCapture, EnableMouseCapture, KeyboardEnhancementFlags,
        PushKeyboardEnhancementFlags,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{collections::HashMap, io::ErrorKind};
use std::{env, error::Error, io};
use tui::{
    backend::{Backend, CrosstermBackend},
    Terminal,
};

pub fn start() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();

    configure_terminal(&mut stdout)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    run_app(&mut terminal)?;

    Ok(())
}

pub fn reset_terminal() -> Result<(), Box<dyn Error>> {
    let stdout = io::stdout();

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

#[cfg(windows)]
fn configure_terminal(stdout: &mut io::Stdout) -> Result<(), Box<dyn Error>> {
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    Ok(())
}

#[cfg(not(windows))]
fn configure_terminal(stdout: &mut io::Stdout) -> Result<(), Box<dyn Error>> {
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES)
    )?;

    Ok(())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>) -> io::Result<()> {
    let mut view = OpenedRepoView::default();
    let args: Vec<String> = env::args().collect();

    if let Some(items) = lib_git_run(terminal, &args) {
        view.repo_commits.table_items = items;
    } else {
        return Err(std::io::Error::new(
            ErrorKind::InvalidInput,
            "Error opening repo select menu",
        ));
    }

    loop {
        let mut run = false;

        terminal.draw(|f| run = view.display_view(f))?;

        if !run {
            return Ok(());
        }
    }
}

fn open_arg_repo(args: &[String]) -> Result<Repository, git2::Error> {
    if let Some(path) = args.get(1) {
        let repo = Repository::open(path)?;
        return Ok(repo);
    } else if let Some(path) = args.get(0) {
        let repo = Repository::open(path)?;
        return Ok(repo);
    }
    let err = git2::Error::new(
        git2::ErrorCode::Directory,
        ErrorClass::None,
        "No directories were provided.",
    );

    Err(err)
}

//TODO: create a open repository view if run dir or arg are not valid

fn lib_git_run<B: Backend>(
    terminal: &mut Terminal<B>,
    args: &[String],
) -> Option<Vec<Vec<String>>> {
    let repo: Repository;

    if let Ok(arg_repo) = open_arg_repo(args) {
        repo = arg_repo;
    } else {
        let selected_repo;
        {
            let mut start_view = StartView::default();

            loop {
                let mut run = true;
                terminal.draw(|f| run = start_view.display_view(f)).ok()?;

                if !run {
                    selected_repo = start_view.repo_selected;
                    break;
                }
            }
        }

        let mut path;

        if let Some(repo_data) = selected_repo {
            path = repo_data.path;
        } else {
            return None;
        }

        let len = path.trim_end_matches(&['\r', '\n'][..]).len();
        path.truncate(len);

        path = path.replace('\\', "/");

        repo = Repository::open(path).ok()?;
    }

    let head = repo.head().ok()?;
    //let name = head.name().unwrap();

    let commit = head.peel_to_commit().ok()?;
    let mut commit_history = Vec::new();
    let mut parent = commit.parents().next();

    let commit_item = extract_commit_data(&commit)?;

    commit_history.push(commit_item);

    while let Some(p_commit) = parent {
        let commit_item = extract_commit_data(&p_commit)?;

        commit_history.push(commit_item);

        parent = p_commit.parents().next();
    }

    let mut url = String::new();

    if let Ok(remote) = repo.find_remote("origin") {
        url = remote.url().unwrap_or_default().to_owned();
    }

    let repo_path = repo
        .path()
        .as_os_str()
        .to_str()
        .unwrap_or("")
        .to_owned()
        .split(".git/")
        .next()
        .unwrap_or_default()
        .to_owned();

    let folders: Vec<&str> = repo_path.split('/').collect();

    let recent_repo = crate::config::repo::Repository {
        path: repo_path.to_owned(),
        name: folders
            .get(folders.len() - 2)
            .unwrap_or(&"UNNAMED")
            .to_string(),
        repo_url: url,
    };

    save_recent_repo(recent_repo);

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

    Some(commit_history)
}

fn extract_commit_data(commit: &Commit) -> Option<Vec<String>> {
    let commit_item = vec![
        commit.message()?.to_owned(),
        commit.author().name()?.to_owned(),
        commit.id().to_string(),
    ];
    Some(commit_item)
}

fn save_recent_repo(repo: crate::config::repo::Repository) -> Option<()> {
    use crate::config::repo::SavedRepositories;

    let mut conf = SavedRepositories::load_or_create_config();
    let mut hash = HashMap::new();

    for recent_repo in conf.recent_repositories {
        hash.insert(recent_repo.path.to_owned(), recent_repo);
    }

    hash.insert(repo.path.to_owned(), repo);

    conf.recent_repositories = hash.into_values().collect();
    conf.save_config().ok()?;

    Some(())
}
