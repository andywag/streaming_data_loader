



use std::{process::Command, fs::File};
use serde::Deserialize;
use tempfile::NamedTempFile;
use std::io::Read;

#[derive(Deserialize, Debug)]

struct ArrowDescriptor {
    pub name:String,
    pub paths:Vec<String>
}

#[derive(Deserialize, Debug)]
struct ArrowFiles {
    pub children:Vec<ArrowDescriptor>
}

impl ArrowFiles {
    pub fn get_locations(self, key:String) -> Option<Vec<String>> {
        for child in self.children {
            if child.name == key {
                return Some(child.paths);
            }
        }
        return None;
    }
}


pub fn download_huggingface_dataset(dataset:&str, typ:Option<&str>, key:&str) -> Option<Vec<String>> {
    let cwd =  "../python";
    let command = "python3";
    
    let file = NamedTempFile::new().unwrap().into_temp_path().as_os_str().to_str().unwrap().to_string();


    //let args =  vec!["download_dataset.py","--dataset", dataset, "--ar", typ, "--store", file.as_str()];
    let args = match typ {
        Some(x) => vec!["download_dataset.py","--dataset", dataset, "--ar", x, "--store", file.as_str()],
        None => vec!["download_dataset.py","--dataset", dataset, "--store", file.as_str()]
    };
    println!("Args {:?}", args);

    let result = Command::new(command).current_dir(cwd).args(args).output().unwrap();
    let stdout = String::from_utf8(result.stdout).unwrap();
    let stderr = String::from_utf8(result.stderr).unwrap();
    

    
    // Close the file, but keep the path to it around.

    println!("Stderr {:?}", stderr);
    println!("Stdout {:?}", stdout);

    let f = File::open(file.as_str());
    let mut buffer = Vec::new();
    // read the whole file
    let _file_size = f.unwrap().read_to_end(&mut buffer);
    let result = serde_pickle::value_from_slice(buffer.as_slice(), Default::default()).unwrap();
    let arrow_file:ArrowFiles = serde_pickle::from_value(result).unwrap();
    //println!("B {:?}", arrow_file.get_locations(key.to_string()));
    let locations =  arrow_file.get_locations(key.to_string());
    return locations;

}

