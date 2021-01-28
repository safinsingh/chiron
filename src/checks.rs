use crate::config::CommandContains;
use anyhow::Result;
use regex::Regex;
use std::{process::Command, str};

/// Wrapper trait for all scorable checks
#[typetag::serde(tag = "type")]
pub trait ScoreableCheck {
	/// Function that returns whether the check has passed or not
	fn score(&self) -> Result<bool>;
}

#[typetag::serde]
impl ScoreableCheck for CommandContains {
	fn score(&self) -> Result<bool> {
		let mut args = self.command.split(' ');
		let cmd = args.next().unwrap();

		let stdout = Command::new(cmd).args(args).output()?.stdout;
		let regex = Regex::new(&self.contains)?;

		Ok(regex.is_match(str::from_utf8(&stdout)?))
	}
}
