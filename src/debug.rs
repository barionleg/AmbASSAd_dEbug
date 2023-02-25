use std::panic::Location;
use std::marker::{Send, Sync};
use std::fmt;

/// The trait that needs to be implemented for using the `watch()` function.
///
/// # Example mockup
/// ```
/// pub struct MockType {}
///
/// impl SubmitMethod for MockType {
///     fn submit(&self, info: &DebugInfo) -> bool {
///         false
///     }
/// }
/// ```

pub trait SubmitMethod: Send + Sync {
    /// The `submit()` function is used for submitting information about a crash of an exectuable
    /// The return value of type 'bool' should indicate wether the
    /// `DebugInfo` is submitted successfully.
    fn submit(&self, info: &DebugInfo) -> bool;

    /// This function will be called by `watch()` if the `submit()` function
    /// returns 'true'.
    /// # Default implementation
    /// ```
    /// fn submission_succeeded(&self, info: &DebugInfo) {
    ///     println!("Thank you for your submission. We will investigate what happened A.S.A.P.");
    /// }
    /// ```
    #[allow(unused_variables)]
    fn submission_succeeded(&self, info: &DebugInfo) {
        println!("Thank you for your submission. We will investigate what happened A.S.A.P.");
    }

    /// This function will be called by `watch()` if the `submit()` function
    /// returns 'false'.
    /// # Default implementation
    /// ```
    /// fn submission_failed(&self, info &DebugInfo) {
    ///     println!("Something went wrong. Our apologies for the inconvenience. This information could not be sent:\n{}", info);
    /// }
    /// ```
    fn submission_failed(&self, info: &DebugInfo) {
        println!("Something went wrong. Our apologies for the inconvenience. This information could not be sent:\n{}", info);
    }
}

/// A struct containing postmortem debug information that will be sent with
/// something that implements the `SubmitMethod` trait, through `watch()`.
pub struct DebugInfo<'a> {
    sha: &'a str,
    description: String,
    info: &'a str,
    location: Option<&'a Location<'a>>
}

impl<'a> DebugInfo<'a> {
    pub fn new(sha: &'a str, desc: String, message: &'a str, panic_location: Option<&'a Location<'a>>) -> DebugInfo<'a> {
        DebugInfo {
            sha: sha,
            description: desc,
            info: message,
            location: panic_location
        }
    }
}

impl<'a> fmt::Display for DebugInfo<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = String::new();

        string.push_str("tree: ");
        string.push_str(self.sha);
        string.push('\n');

        string.push_str("at: ");

        match self.location {
            Some(location) => string.push_str(&location.to_string()),
            None => string.push_str("<UNKOWN>")
        }

        string.push('\n');

        string.push_str("Panic details: \n");
        string.push_str(self.info);
        string.push('\n');

        string.push_str("Summary: \n");
        string.push_str(&self.description);

        try!(write!(f, "{}", string));
        Ok(())
    }
}
