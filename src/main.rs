mod bar;
mod categorize_extensions;
mod languages;
mod licenses;
mod readdirs;
use bar::bar;
use categorize_extensions::categorize_extensions;
use color_please::{reset_all, set_fg, Color};
use licenses::*;
use readdirs::readdirs;
use std::env;
use std::fs::{read_dir, read_to_string};
use std::path::PathBuf;
use std::process;

fn main() {
    let cmdline_args: Vec<String> = env::args().collect();
    let path_to_project = match cmdline_args.get(1) {
        Some(n) => n,
        None => {
            eprintln!("Usage: projlyzer /path/to/project");
            process::exit(1);
        }
    };
    let files = readdirs(path_to_project);
    let clone_files = files.clone();
    bar(&categorize_extensions(files));
    reset_all();
    println!("Total count of files: {}", clone_files.len());
    let license_text: String;
    let readdir_iterator = match read_dir(path_to_project) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("An error occured: {}", e);
            process::exit(1);
        }
    };
    for i in readdir_iterator {
        let lower_filename = i
            .as_ref()
            .unwrap()
            .file_name()
            .to_string_lossy()
            .to_lowercase();
        if (lower_filename.starts_with("license") || lower_filename.starts_with("copying")) && !lower_filename.starts_with("licenses") {
            let mut license_file = PathBuf::from(path_to_project);
            license_file.push(i.unwrap().file_name().to_str().unwrap());
            license_text = match read_to_string(license_file) {
                Ok(res) => res,
                Err(e) => {
                    eprintln!("An error occured: {}", e);
                    process::exit(1);
                }
            };
            println!("License: {}", find_license(&license_text));
            break;
        }
    }
    let mut path_to_check_git = PathBuf::from(path_to_project);
    path_to_check_git.push(".git");
    set_fg(Color::Color256(202));
    print!("Includes a git repository? ");
    reset_all();
    if path_to_check_git.is_dir() {
        set_fg(Color::BrightGreen);
        println!("Yes");
    } else {
        set_fg(Color::BrightRed);
        println!("No");
    }
    reset_all();
}
