
use std::{env, path::Path};

use anyhow::{Context, Result};
use git2::{
    AutotagOption, Cred, FetchOptions, RemoteCallbacks, Repository,
    build::RepoBuilder,
};

use crate::config::profile::Profile;

fn expand_home(path: &str) -> String {
    match env::var("HOME") {
        Ok(home) => path.replacen('~', &home, 1),
        Err(_) => path.to_string(),
    }
}

fn callbacks(profile: &Profile) -> RemoteCallbacks<'_> {
    let mut cb = RemoteCallbacks::new();
    cb.credentials(move |_url, username_from_url, _allowed| {
        let keypath = expand_home(&profile.keys.auth.0);
        Cred::ssh_key(
            username_from_url.unwrap_or("git"),
            None,
            Path::new(&keypath),
            None,
        )
    });
    cb
}

fn fetch_options(profile: &Profile) -> FetchOptions<'_> {
    let mut fo = FetchOptions::new();
    fo.remote_callbacks(callbacks(profile));
    fo.download_tags(AutotagOption::All);
    fo
}

pub fn clone(
    profile: &Profile,
    url: &str,
    dest: &Path,
    bare: bool,
) -> Result<Repository> {
    if let Some(parent) = dest.parent() {
        std::fs::create_dir_all(parent)
            .context("while creating the destination parent directory")?;
    }

    let mut builder = RepoBuilder::new();
    builder.bare(bare);
    builder.fetch_options(fetch_options(profile));

    let git_dir = if bare {
        dest.join(".git")
    } else {
        dest.to_path_buf()
    };

    builder
        .clone(url, &git_dir)
        .context("while cloning repository")
}

pub fn update(profile: &Profile, dest: &Path) -> Result<()> {
    let repo = Repository::open(dest)
        .context("while opening the repository")?;

    {
        let mut remote = repo
            .find_remote("origin")
            .context("while finding the origin remote")?;
        remote
            .fetch::<&str>(&[], Some(&mut fetch_options(profile)), None)
            .context("while fetching from origin")?;
    }

    let head = repo
        .find_reference("FETCH_HEAD")
        .context("while reading FETCH_HEAD")?;
    let fetched = repo
        .reference_to_annotated_commit(&head)
        .context("while resolving FETCH_HEAD")?;
    let (analysis, _) = repo
        .merge_analysis(&[&fetched])
        .context("while analyzing the merge")?;

    if analysis.is_up_to_date() {
        return Ok(());
    }
    if !analysis.is_fast_forward() {
        anyhow::bail!("upstream has diverged; refusing a non-fast-forward update");
    }

    let refname = {
        let head = repo.head().context("while reading HEAD")?;
        head.name()
            .context("while reading the HEAD reference name")?
            .to_string()
    };
    let mut reference = repo
        .find_reference(&refname)
        .context("while finding the branch reference")?;
    reference
        .set_target(fetched.id(), "uni: fast-forward")
        .context("while moving the branch")?;
    repo.set_head(&refname).context("while setting HEAD")?;
    repo.checkout_head(Some(git2::build::CheckoutBuilder::new().force()))
        .context("while checking out the new HEAD")?;
    Ok(())
}
