use std::time::Duration;

use clap::ArgMatches;

use crate::errors::GitRelatesTo;

pub struct Args {
    matches: ArgMatches,
}

impl From<ArgMatches> for Args {
    fn from(matches: ArgMatches) -> Self {
        Self { matches }
    }
}
use miette::{IntoDiagnostic, Result};
use mit_commit_message_lints::console::completion::Shell;

impl Args {
    pub(crate) fn issue_number(&self) -> Result<&str> {
        self.matches
            .value_of("issue-number")
            .map_or_else(|| Err(GitRelatesTo::NoRelatesToMessageSet.into()), Ok)
    }

    pub(crate) fn timeout(&self) -> Result<Duration> {
        self.matches
            .value_of("timeout")
            .map_or_else(|| Err(GitRelatesTo::NoTimeoutSet.into()), Ok)
            .and_then(|timeout| timeout.parse().into_diagnostic())
            .map(|timeout: u64| timeout * 60)
            .map(Duration::from_secs)
    }

    pub fn completion(&self) -> Option<Shell> {
        self.matches.value_of_t::<Shell>("completion").ok()
    }
}
