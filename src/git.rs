use git2::{Repository, Signature};
use std::path::Path;

pub fn push_to_github(
    repo_path: &Path,
    file_path: &Path,
    message: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let repo = Repository::open(repo_path)?;

    // Ajouter le fichier à l'index
    let mut index = repo.index()?;
    index.add_path(file_path.strip_prefix(repo_path)?)?;
    index.write()?;

    // Créer un commit
    let oid = index.write_tree()?;
    let tree = repo.find_tree(oid)?;
    let parent_commit = repo.head()?.peel_to_commit()?;
    let sig = Signature::now("xor_claude", "xor@claude.ai")?;

    repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent_commit])?;

    // Push vers origin
    let mut remote = repo.find_remote("origin")?;
    remote.push(&["refs/heads/main"], None)?;

    Ok(())
}
