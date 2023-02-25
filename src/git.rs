extern crate git2;

/// This method can return a String that represents the current git tree in
/// the project's working directory.
/// The String also indicates wether the tree is dirty.
pub fn sha() -> Option<String> {
    match git2::Repository::discover("./") {
        Ok(repo) => {
            let _remotes = match repo.remotes() {
                Ok(array) => array,
                Err(e) => panic!("No remotes found. Details: {}", e)
            };

            let head = match repo.head() {
                Ok(reference) => match reference.peel_to_tree() {
                    Ok(tree) => tree,
                    Err(e) => panic!("Could not peel head to tree! Details: {}", e)
                }
                Err(e) => panic!("Could not find head. Details: {}", e)
            };

            let mut diff_option = git2::DiffOptions::new();
            diff_option.include_untracked(true);

            match repo.diff_tree_to_workdir(Some(&head), Some(&mut diff_option)) {
                Ok(_) => return Some(head.id().to_string() + " (dirty)"),
                Err(e) => panic!("Could not create diff. Details: {}", e)
            }
        },
        Err(e) => {
		println!("No git repo found. Details: {}", e);
		None
	}
    }
}
