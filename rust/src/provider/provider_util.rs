use serde_json::Value;


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

