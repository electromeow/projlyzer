use std::collections::HashMap;
use std::fs;
use crate::languages::{Language, extensions};


pub fn categorize_extensions(list_of_files: Vec<String>) -> HashMap<Language, u128> {
  let mut languages_usage: HashMap<Language, u128> = HashMap::new();
  let exts: HashMap<&str,Language> = extensions();
  for i in list_of_files {
    let content = match fs::read_to_string(i.as_str()) {
      Ok(cont) => cont,
      Err(_) => continue
    };
    let extension;
    let filename= i.split('/').last().unwrap();
    if !filename.contains('.') {
      extension = "";
      if filename.to_lowercase().ends_with("file")||filename.to_lowercase() == "license"||filename.to_lowercase() == "copying" {
        continue;
      }
    }
    else {
      extension = i.split('.').last().unwrap();
    }

    let language = match exts.get(extension) {
      Some(x) => x,
      None => continue
    };
    if !languages_usage.contains_key(language) {
      languages_usage.insert((*language).clone(), 0 as u128);
    }
    *languages_usage.get_mut(&language).unwrap() += content.lines().collect::<Vec<&str>>().len() as u128;
  }
  languages_usage
}