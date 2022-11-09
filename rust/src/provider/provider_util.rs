
pub fn create_text(line:String, finder:&str) -> String{
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
    return new_text.into_iter().collect();

}
