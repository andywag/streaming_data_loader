



use std::{process::Command, fs::File};
use serde::Deserialize;
use tempfile::NamedTempFile;
use std::io::Read;



#[derive(Deserialize, Debug)]

struct ArrowDescriptor {
    pub name:String,
    pub paths:Vec<String>,
    pub num_rows:u32
}

#[derive(Deserialize, Debug)]
pub struct ArrowFiles {
    children:Vec<ArrowDescriptor>
}

impl ArrowFiles {
    pub fn get_locations(self, key:String) -> Option<(Vec<String>,u32)> {
        for child in self.children {
            if child.name == key {
                return Some((child.paths, child.num_rows));
            }
        }
        return None;
    }
}

// Download a Huggingface dataset
// Uses python to download the dataset and create a pickle file with information
// Loads the pickle file to get arrow file location and number of rows in arrow file
pub fn download_huggingface_dataset(dataset:String, typ:Option<String>) -> Option<ArrowFiles> {
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
    //println!("B {:?}", arrow_file);
    //let locations =  arrow_file.get_locations(key.to_string());
    //return locations;
    return Some(arrow_file);

}

pub fn create_hugging_description(dataset:String, extra:Option<String>, operation:String) -> (String, u32) {
    let arrow_files = download_huggingface_dataset(dataset, extra).unwrap();
    let arrow_train = arrow_files.get_locations(operation).unwrap();
    let arrow_location = arrow_train.0[0].to_owned();
    let arrow_length = arrow_train.1;
    (arrow_location, arrow_length)
}

