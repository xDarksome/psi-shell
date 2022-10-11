use std::ffi::OsString;
use std::str::FromStr;
use std::{fmt, process};

pub type Handles = std::collections::HashMap<OsString, process::Child>;

pub fn exec<T: FromStr>(s: &str) -> Result<T, String>
where
    T::Err: fmt::Display,
{
    let output = command(s)?.output().map_err(|e| format!("{s}: {e}"))?;

    let stderr = String::from_utf8(output.stderr)
        .map_err(|e| format!("{s}: stderr has non-UTF-8 symbols: {e}"))?;

    if !stderr.is_empty() {
        return Err(format!("{s}: stderr: {stderr}"));
    }

    String::from_utf8(output.stdout)
        .map_err(|e| format!("{s}: stdout has non-UTF-8 symbols: {e}"))?
        .trim()
        .parse()
        .map_err(|e| format!("{s}: parse stdout: {e}"))
}

pub fn exec_json<T: for<'a> serde::de::Deserialize<'a>>(s: &str) -> Result<T, String> {
    let output: String = exec(s)?;
    serde_json::from_str(&output).map_err(|e| format!("{s}: Deserialize JSON: {e}"))
}

pub fn toggle(procs: &mut Handles, s: &str) -> Result<(), String> {
    let mut cmd = command(s)?;
    let program = cmd.get_program().to_owned();

    if let Some(mut handle) = procs.remove(&program) {
        handle.kill().map_err(|e| format!("Kill {program:?}: {e}"))
    } else {
        let handle = cmd.spawn().map_err(|e| format!("Spawn {program:?}: {e}"))?;
        Ok(drop(procs.insert(program.to_owned(), handle)))
    }
}

fn command(s: &str) -> Result<process::Command, String> {
    let mut parts = s.split(' ');
    let program = parts.next().ok_or_else(|| "Missing program name".to_string())?;
    let mut cmd = process::Command::new(program);
    let _ = cmd.args(parts);
    Ok(cmd)
}
