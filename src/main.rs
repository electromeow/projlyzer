mod readdirs;
mod languages;
mod categorize_extensions;
mod bar;
use categorize_extensions::categorize_extensions;
use readdirs::readdirs;
use bar::bar;
use std::env;
use std::process;

fn main() {
    let cmdline_args: Vec<String> = env::args().collect();
    let path_to_project = match cmdline_args.get(1){
        Some(n) => n,
        None => {
            eprintln!("Usage: projlyzer /path/to/project");
            process::exit(1);
        }
    };
    let files = readdirs(path_to_project);
    let clone_files = files.clone();
    bar(&categorize_extensions(files));
    println!("Total count of files: {}", clone_files.len());
}
