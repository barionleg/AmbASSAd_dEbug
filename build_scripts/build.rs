mod release;

#[cfg(debug_assertions)]
fn main() {}

#[cfg(not(debug_assertions))]
fn main() {
    release::repo_check();
}
