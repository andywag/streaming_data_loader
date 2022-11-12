use serde_json::Value;
use std::{path::PathBuf, str::FromStr};
use cached_path::Cache;

pub enum DownloadType {
    Zstd,
    Gzip,
    Error
}

pub fn get_download_type(path:&String) -> DownloadType{
    if &path[path.len()-4..path.len()] == ".zst" {
        return DownloadType::Zstd
    }   
    else if &path[path.len()-3..path.len()] == ".gz"{
        return DownloadType::Gzip
    }
    else {
        DownloadType::Error
    }
}


pub fn get_cached_file(cache_path:String, url:&String, offline:bool) -> Option<PathBuf> {

    let base_path = PathBuf::from_str(cache_path.as_str()).unwrap();

    let cache = Cache::builder()
    .dir(base_path)
    .connect_timeout(std::time::Duration::from_secs(20))
    .freshness_lifetime(10000000)
    .offline(offline)
    //.progress_bar(progress_bar)
    .build()
    .unwrap();

    let result = cache.cached_path(url.as_str());
    match result {
        Ok(x) => Some(x),
        Err(_) => None,
    }
}




// Returns the base name of the URL
pub fn split_path(path:String) -> Option<String> {
    let split_path:Vec<&str> = path.split("/").collect();
    let filename = split_path[split_path.len()-1];
    let mut base = filename.split(".");
    let basename = base.next().map(|e| e.to_string());
    basename
}

// Parses the json line and returns a text string
pub fn create_json_text(line:String, tag:&str) -> Option<String> {
    let v: Value = serde_json::from_str(line.as_str()).unwrap();
    v[tag].as_str().map(|e| e.to_string())
}

// Extract the text from a json file with the string associated with finder
// This method should be faster than doing a json decode
pub fn create_text(line:String, finder:&str) -> Option<String>{
    let search:Vec<char> = finder.chars().collect();
    let mut index:usize = 0;
    let mut sp:usize = 0;
    let mut escape = false;
    let mut new_text = Vec::<char>::with_capacity(2048);
    let mut wait = 0;
    for (i,c) in line.chars().enumerate() {
        if sp == 0 {
            if c == search[index] {
                index += 1;
                if index == search.len() {
                    sp = i;
                }
            }
            else {
                index = 0;
            }
        }
        else {
            if sp != 0 && c == '"' && !escape {
                break;
            }
            else {
                if escape && c == 'u' {
                    wait = 4;
                }
                else if wait > 0 {
                    wait = wait - 1;
                }
                if c != '\\' && wait == 0{
                    new_text.push(c);
                }
            }
        }
        escape = c == '\\';
    }
    //println!("Here {} {}", sp, ep);
    if new_text.len() > 32 {
        return Some(new_text.into_iter().collect());
    }
    else {
        return None;
    }

}

