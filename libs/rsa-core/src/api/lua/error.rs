use color_eyre::owo_colors::OwoColorize;
use color_eyre::{IndentedSection, Section, SectionExt};
use eyre::{ErrReport, Report, Result};
use mlua::Error;
use regex::{Captures, Regex};

pub trait LuaError<T> {
	fn lua_err(self) -> Result<T, ErrReport>;
}

impl<T> LuaError<T> for mlua::Result<T> {
	fn lua_err(self) -> Result<T, ErrReport> {
		match self {
			Err(err) => {
				match err {
					Error::RuntimeError(test) => {
						let (reason, traceback) = test.split_once("stack traceback:\n").unwrap();
						return Err(Report::msg(reason.trim().to_string()).section(stacktrace(traceback)));
					}
					Error::CallbackError { traceback, cause } => {
						let traceback = traceback.trim_start_matches("stack traceback:\n");
						return Err(Report::new(cause).section(stacktrace(traceback)));
					}
					_ => {}
				}
				Err(Report::new(err))
			}
			Ok(t) => Ok(t),
		}
	}
}

fn stacktrace(traceback: &str) -> IndentedSection<String, String> {
	let result = Regex::new(r#"\[string "([\s\S]*?)"]"#).unwrap();
	let mut traces: Vec<&str> = traceback.rsplit('\n').collect();
	traces.reverse();
	let mut out = String::new();
	for (i, trace) in traces.into_iter().enumerate() {
		let text = trace.trim();
		let text = result.replace_all(text, |captures: &Captures| {
			format!("[{}]", &captures[1])
		}).to_string();

		let (location, value) = text.split_once(": ").unwrap();

		out.push_str(&i.purple().to_string());
		out.push_str(": ");

		out.push_str(&location.cyan().to_string());
		out.push_str(": ");

		out.push_str(&value.bright_white().bold().to_string());
		out.push('\n');
	}

	out.header("Lua stacktrace".to_string())
}