use std::fs::read_dir;
use std::path::Path;
use std::process;

pub fn readdirs(path: &str) -> Vec<String> {
    let mut list_of_files = Vec::<String>::new();
    let readdir_iterator = match read_dir(Path::new(path)) {
        Ok(res) => res,
        Err(e) => {
            eprintln!("An error occured: {}", e);
            process::exit(1);
        }
    };
    for i in readdir_iterator {
        let ref_i = i.as_ref().unwrap();
        let meta = ref_i.metadata().unwrap();
        let path = ref_i.path();
        if meta.is_dir() {
            for i2 in readdirs(path.to_str().unwrap()) {
                list_of_files.push(i2);
            }
        }
        if meta.is_file() {
            list_of_files.push(i.unwrap().path().to_string_lossy().into_owned());
        }
    }
    list_of_files
}
