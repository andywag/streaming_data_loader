use std::{path::PathBuf, fs::File, io::BufWriter};
use std::io::{Write};

use serde::Serialize;
use std::process::{Command};

#[derive(Serialize)]
pub struct Data {
    pub text:String
}

pub struct CacheWriter {
    pub location:PathBuf,
    pub writer:BufWriter<File>
}

impl CacheWriter {

    pub fn new(path:PathBuf) -> Self {
        let new_location = path.to_owned().with_extension("json");
        log::info!("Creating File {:?}", &new_location);

        let file = File::create(new_location).unwrap();
        let writer = BufWriter::new(file);
        Self {
            location:path,
            writer:writer
        }
    }

    pub fn write_line(&mut self, data:String) {
        let real_data = Data{text:data};
        let mut result = serde_json::to_vec(&real_data).unwrap();
        result.extend(b"\n");
        let _ = self.writer.write_all(&result[0..result.len()]);
    }

    pub fn finish(&mut self) {
        let _ = self.writer.flush();
        let parent_folder = self.location.parent().unwrap().as_os_str();
        let base_location = self.location.to_owned().with_extension("json").as_os_str().to_owned();
        let args = vec![base_location];
        let result = Command::new("zstd").current_dir(parent_folder).args(args).output();
        let stderr = &result.unwrap().stdout[..]; 
        let std_string = std::str::from_utf8(stderr).unwrap();
        let _fail = std_string.find("FAILED");
        
    }

}

pub fn existing_cache_file(cached_path:&PathBuf) -> Option<PathBuf> {
    let zstd_file = cached_path.with_extension("json.zst");
    
    if zstd_file.exists() {
        Some(zstd_file)
    }
    else {
        None
    }
}
