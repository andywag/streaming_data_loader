



use std::{process::Command, fs::File, rc::Rc};
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


pub fn download_huggingface_dataset(dataset:String, typ:Option<String>, key:String) -> Option<Vec<String>> {
    let cwd =  "../python";
    let command = "python3";
    
    let file = NamedTempFile::new().unwrap().into_temp_path().as_os_str().to_str().unwrap().to_string();
    
    let prog = "download_dataset.py".to_string();
    let dataset_c = "--dataset".to_string();
    let ar_c = "--ar".to_string();
    let store_c = "--store".to_string();

    //let args =  vec!["download_dataset.py","--dataset", dataset, "--ar", typ, "--store", file.as_str()];
    let args = match typ {
        Some(x) => {
            //astr = &typ.unwrap()[..];  // take a full slice of the string
            vec![prog, dataset_c, dataset, ar_c, x, store_c, file.clone()]
        }
        None => vec![prog, dataset_c, dataset, store_c, file.clone()]
    };
    println!("Args {:?}", args);

    

    let result = Command::new(command).current_dir(cwd).args(args).output().unwrap();
    let stdout = String::from_utf8(result.stdout).unwrap();
    let stderr = String::from_utf8(result.stderr).unwrap();
    

    
    // Close the file, but keep the path to it around.

    println!("Stderr {:?}", stderr);
    println!("Stdout {:?}", stdout);

    let f = File::open(file);
    let mut buffer = Vec::new();
    // read the whole file
    let _file_size = f.unwrap().read_to_end(&mut buffer);
    let result = serde_pickle::value_from_slice(buffer.as_slice(), Default::default()).unwrap();
    let arrow_file:ArrowFiles = serde_pickle::from_value(result).unwrap();
    //println!("B {:?}", arrow_file.get_locations(key.to_string()));
    let locations =  arrow_file.get_locations(key.to_string());
    return locations;

}

