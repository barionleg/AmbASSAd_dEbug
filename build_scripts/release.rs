#![cfg(not(debug_assertions))]

extern crate git2;

pub fn repo_check() {
    match git2::Repository::discover("./") {
        Ok(repo) => {
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
                Ok(ref diff) => diff_check(diff),
                Err(e) => panic!("Could not create diff. Details: {}", e)
            }
        },
        Err(e) => println!("No git repo found. Details: {}", e)
    }
}

fn diff_check(diff: &git2::Diff) {
    match diff.stats() {
        Ok(ref stats) if stats.files_changed() > 0 => {
            println!("cargo:warning=ambassade-debug: Current repository is dirty, meaning that debug info from this build will be less reliable.");
            println!("cargo:warning=ambassade-debug: Changed files: {:?}", stats.files_changed());
        },
        Ok(_) => {},
        Err(e) => panic!("Could not read stats from git tree. Details: {}", e.to_string())
    }
}
