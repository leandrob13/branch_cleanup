use app::commands;
use std::io::Error;
use std::path::PathBuf;
use std::env;

pub mod app;

fn main() {

    let args: Vec<String> = env::args().collect();
    let dir= &args[1];

    commands::get_dirs(&dir)
        .map(|d: Vec<PathBuf>| -> Vec<PathBuf> {
            d.into_iter().filter_map(|r| commands::get_git_dirs(&r)).collect()
        }).and_then(|gd: Vec<PathBuf>| -> Result<String, Error> {
            gd.iter().for_each(|r: &PathBuf| -> () {
                println!("RESULT {}", r.to_str().unwrap());
                let out = commands::execute_command("git", &r.to_str().unwrap().to_string(), vec!["checkout", "master"]);
                match out {
                    Ok(res) => println!("SUCCESS {}", res),
                    Err(e) => println!("FAILED {}", e)
                }
            });
            Ok("ok".to_string())
        }).unwrap();
}
