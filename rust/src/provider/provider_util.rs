use serde_json::Value;

#[derive(Debug)]
pub enum DownloadType {
    Zstd,
    Gzip,
    Error
}


pub fn get_local_path(path:&String) -> String {
    path.replace("http", "").replace(":", "").replace("/", "_").replace(".", "_")
    
}

pub fn is_network(path:&String) -> bool {
    path.contains("http")
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



// Returns the base name of the URL
pub fn split_path(path:String) -> Option<String> {
    let split_path:Vec<&str> = path.split("/").collect();
    let filename = split_path[split_path.len()-1];
    let mut base = filename.split(".");
    let basename = base.next().map(|e| e.to_string());
    basename
}

// Parses the json line and returns a text string
pub fn create_json_python_text(line:String, tag:&str) -> Option<String> {
    let v: Value = serde_json::from_str(line.as_str()).unwrap();
    let repo_language = v["meta"]["file_name"].as_str();
    //log::info!("Repo Language {:?}", repo_language);
    if repo_language.is_none() { // meta -- filename not included after filter
        v[tag].as_str().map(|e| e.to_string())
    }
    else if repo_language.is_some() && repo_language.unwrap().contains(".py") {
        //log::info!("Found File {:?}", repo_language);
        v[tag].as_str().map(|e| e.to_string())
    }
    else {
        None
    }
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


pub fn clean_t5(text:String) -> Option<String> {
    let chars = text.chars();
    let mut word = String::with_capacity(16);
    let mut sentence = Vec::<String>::with_capacity(128);
    let mut result = Vec::<Vec<String>>::with_capacity(32);

    let mut lorem = false;

    for c in chars {
        if c == '{' {
            return None;
        }
        else if c == ' ' || c == '\n' || c == '\t' || c == '\r' { // whitespace -- create word
            if word == "lorem" {
                lorem = true;
            }
            if lorem && word == "ipsum" {
                return None;
            }
            if word.len() > 0 && word != "javascript" {
                    word.push(' ');
                    sentence.push(word.clone());
            }
            word.clear();
        } 
        else if c == '.' || c == '!' || c == '?' { // sentence completion
            word.push(c);
            word.push(' ');
            if sentence.len() > 3 {
                result.push(sentence.clone());
            }
            word.clear();
            sentence.clear();
        }
    }
    if result.len() > 5 {
        let fin:String = result.into_iter().flatten().collect();
        Some(fin);   
    }

    None

}