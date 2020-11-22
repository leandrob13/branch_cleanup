use std::process::{Command, Output};
use std::io::Error;
use std::string::FromUtf8Error;
use std::path::PathBuf;
use std::fs::{ReadDir, DirEntry};
use std::io;

pub fn execute_command(program: &str, dir: &String, args: Vec<&str>) -> Result<String, String> {
    let mut command = Command::new(program);
    if !args.is_empty() {
        command.args(args);
    }
    command
        .current_dir(&dir)
        .output()//.map(|o| o.stdout)
        .map_err(|e: Error| -> String {format!("Error ocurred: {}", e.to_string())})
        .and_then(|r: Output| -> Result<String, String> {
            match String::from_utf8(r.stderr).map_err(map_error_message) {
                Ok(err) if err.contains("Already on 'master'") => Ok(err),
                Ok(err) if err.is_empty() =>
                    String::from_utf8(r.stdout).map_err(map_error_message),
                Ok(err) => Err(err),
                res=> res
            }
        })
}

fn map_error_message(e: FromUtf8Error) -> String {
    format!("Error ocurred: {}", e.to_string())
}

pub fn get_dirs(dir: &String) -> Result<Vec<PathBuf>, Error> {

    PathBuf::from(dir)
        .read_dir()
        .and_then(|rd: ReadDir| -> io::Result<Vec<DirEntry>> { rd.collect() })
        .map(|v: Vec<DirEntry>| ->  Vec<PathBuf> {
            v.into_iter()
                .filter_map(|d| Option::Some(d.path()).filter(|p| p.is_dir()))
                .collect()
        })
}

pub fn get_git_dirs(dir: &PathBuf) -> Option<PathBuf> {
    dir.read_dir()
        .and_then(|rd: ReadDir| -> Result<Vec<DirEntry>, Error> {rd.collect()})
        .ok()
        .filter(|x| x.iter().any(|z| z.file_name().eq(".git")))
        .map(|_| dir.into())
}