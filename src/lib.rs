/// This module contains all the tools that the `watch()` function needs.
pub mod debug;
mod git;

use std::{io, panic};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

/// This function is used to send debug information, using the `submit()` function
/// from the `method` parameter when the Rust application panics.
pub fn watch<F: 'static>(method: F) where F: debug::SubmitMethod {
    panic::set_hook(Box::new(move |panic_info| {
        let mut input = String::new();

        println!("Program crashed unexpectedly! Would you like to submit a report [Y/n]?");

        match io::stdin().read_line(&mut input) {
            Ok(_) if input.as_str() == "n\n" => return (),
            _ => {}
        }

        println!("Please describe briefly what happened.");

        let desc = match io::stdin().read_line(&mut input) {
            Ok(_) => input.clone(),
            Err(_) => String::from("Error while describing bug.")
        };

        let sha = match git::sha() {
            Some(tree) => tree,
            None => String::from("None")
        };

        let dbg = debug::DebugInfo::new (
            &sha,
            desc,
            panic_info.payload().downcast_ref::<&str>().unwrap(),
            panic_info.location()
        );

        match method.submit(&dbg) {
            true => method.submission_succeeded(&dbg),
            false => method.submission_failed(&dbg)
        }
    }));
}
