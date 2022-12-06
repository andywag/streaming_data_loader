use std::{collections::HashMap};

use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]



pub struct ContextLookupNew {
    pub to_key:HashMap<String, usize>,
    pub from_key:HashMap<usize, String>,
    size:usize,
    current:usize
}


impl ContextLookupNew {
    pub fn new(s:usize) -> Self {
        Self {
            to_key: HashMap::<String,usize>::with_capacity(s),
            from_key: HashMap::<usize,String>::with_capacity(s),
            size:s,
            current:0
        }
    }

    pub fn from_file(file_path:&str) -> Self {
        let contents = fs::read_to_string(file_path).unwrap();
        let split = contents.split("\n");
        let mut to_key = HashMap::<String,usize>::with_capacity(2048);
        let mut from_key = HashMap::<usize,String>::with_capacity(2048);
        let mut count:usize = 0;
        for s in split {
            let f = s.split(" ").next().unwrap();
            to_key.insert(f.to_string(), count);
            from_key.insert(count, f.to_string());
            count += 1;
        }
        //log::info!("Creating  ... {:?}", to_key);
        Self {
            to_key: to_key,
            from_key: from_key,
            size:count,
            current:count
        }
    }


    pub fn put_data(&mut self, data:&str) -> Option<u32> {
        let result = self.to_key.get(data);
        match result {
            Some(x) => return Some(x.clone() as u32),
            None => {
                if self.current < self.size {
                    self.to_key.insert(data.to_string(), self.current);
                    self.from_key.insert(self.current, data.to_string()) ;
                    self.current += 1;
                    Some(self.current as u32 - 1)
                }
                else {
                    None
                }
            },
        }
    }

    pub fn get_data(&self, data:&str) -> Option<u32> {
        let result = self.to_key.get(data).map(|s|s.to_owned() as u32);
        result
    }

   
}



pub struct ContextStoreNew<'a> {
    global_store:&'a ContextLookupNew,
    project_store:&'a ContextLookupNew,
    context:Vec<ContextLookupNew>,
    local_size:usize,
    offset:usize, 
}

impl <'a>ContextStoreNew<'a> {
    pub fn new(global_store:&'a ContextLookupNew, project_store:&'a ContextLookupNew, local_size:usize, offset:usize) -> Self{
        Self {
            global_store:global_store,
            project_store:project_store,
            context:vec![ContextLookupNew::new(local_size)],
            local_size:local_size,
            offset:offset
        }
    }

    pub fn get_id(&self, location:&(u32,u32)) -> u32 {
        if location.0 == 0 {
            (self.offset as u32 + location.1) as u32
        }
        else {
            ((location.0 - 2)*self.local_size as u32 + location.1 + self.offset as u32) as u32
        }

    }

    pub fn len(&self) -> usize {
        return self.context.len() + 2;
    }

    pub fn pop_context(&mut self) {
        self.context.pop();
    }

    pub fn push_context(&mut self) {
        let lut = ContextLookupNew::new(self.local_size);
        self.context.push(lut);
    }


    /// Returns the level and index of the input data
    fn get_data(&mut self, text:&str) -> Option<(u32,u32)> {
        let l = self.context.len();
        for x in 0..l { // Search All of the Context Levels
            let result = self.context[l-x-1].get_data(text);
            if result.is_some() {
                return Some(( (l-x+1) as u32,result.unwrap()));
            }
        }
        // Search over local context
        let local =  self.project_store.get_data(text);
        if local.is_some() {
            return Some((1, local.unwrap()));
        }
        // Search over Global Context
        let glob = self.global_store.get_data(text);
        let r = glob.map(|s|(0,s));
        //log::info!("Couldn't Find Data {:?} {:?}", text, self.global_store);
        r

    }

    // Put identifier onto the global context of file context if doesn't fit
    pub fn put_local(&mut self, text:&str) -> (u32, u32) {
        if self.context.len() >= 1 {
            let result = self.context[0].put_data(text);
            match result {
                Some(x) => (self.context.len() as u32 +1, x),
                None => (0,0),
            }
        }
        else {
            (0,0)
        }
    }

    pub fn get_or_put_local(&mut self, text:&str) -> (u32, u32) {
        let result = self.get_data(text);
        if result.is_some() {
            result.unwrap()
        }
        else {
            self.put_local(text)
        }
    }






}

