use std::env;
use std::process::{Command, exit};
use std::fs;
use std::path::Path;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <directory> <depth>", args[0]);
        exit(1);
    }

    let directory = &args[1];
    let depth: usize = args[2].parse().expect("Depth must be a number");
    update_repos(directory, depth, 0);
}

fn update_repos(directory: &str, max_depth: usize, current_depth: usize) {
    if current_depth > max_depth {
        return;
    }
    let paths = fs::read_dir(directory).unwrap();

    for path in paths {
        let path = path.unwrap().path();
        if path.is_dir() {
            if Path::new(&format!("{}/.git", path.display())).exists() {
                println!("Updating {}", path.display());
                let output = Command::new("git")
                    .arg("-C")
                    .arg(path.to_str().unwrap())
                    .arg("pull")
                    .output()
                    .expect("Failed to execute command");

                // Check if the command was successful
                if output.status.success() {
                    io::stdout().write_all(&output.stdout).unwrap();
                } else {
                    io::stderr().write_all(&output.stderr).unwrap();
                }
            } else {
                update_repos(path.to_str().unwrap(), max_depth, current_depth + 1);
            }
        }
    }
}
