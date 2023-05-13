use crate::config::Config;
use crate::link;

pub enum CliAction {
    NotFound(String),
    Push(String),   // source
    Pull(String), // target
    Clone(String), // repo
}

pub fn evaluate(action: CliAction) -> Result<(), String> {
    match action {
        CliAction::Push(s) => push(s),
        CliAction::Pull(s) => pull(s),
        CliAction::Clone(s) => clone(s),
        CliAction::NotFound(s) => Err(format!("command '{s}' not recognized.")),
    }?;

    return Ok(());
}

// - create link from source file
// - move source to target path
// - pull link
fn push(source: String) -> Result<(), String> {
    if source.len() == 0 {
        return Err("not enough arguments provided.".to_string());
    }
    let link = link::Link::from_source(source)?;
    link.push_source()?;
    link.register()?;
    link.pull_target()?;

    Ok(())
}

fn pull(target: String) -> Result<(), String> {
    if target.len() == 0 {
        let config = Config::new()?;
        for l in &config.user.items {
            l.pull_target()?;
        }
        return Ok(());
    }
    let link = link::Link::from_target(target)?;
    link.pull_target()?;

    Ok(())
}

// - clone repo
fn clone(_repo: String) -> Result<(), String> {
    todo!()
}
