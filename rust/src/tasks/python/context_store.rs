use std::{collections::{HashSet, HashMap}};

use serde::{Deserialize, Serialize};
use std::fs;

use super::python_tokenizer::Token;

/// Storage class to hold a set of variables in a set
#[derive(Debug, Serialize, Deserialize)]
pub struct ContextSet {
    pub items:HashSet<String>,
    pub id:u32
}
impl ContextSet {
    pub fn new(id:u32) -> Self {
        Self {
            items:HashSet::<String>::with_capacity(1024),
            id
        }
    }

    pub fn get_data(&self, data:&String) -> Option<u32> {
        if self.items.contains(data) {
            Some(self.id)
        }
        else {
            None
        }
    }
}

/// Struct which contains a list of variables and their index in the map
#[derive(Debug, Serialize, Deserialize)]
struct ContextMap {
    pub to_key:HashMap<String, usize>,
    pub from_key:HashMap<usize, String>,
    size:usize,
    current:usize
}


impl ContextMap {
    pub fn new(s:usize) -> Self {
        Self {
            to_key: HashMap::<String,usize>::with_capacity(s),
            from_key: HashMap::<usize,String>::with_capacity(s),
            size:s,
            current:0
        }
    }

    /// Method which loads up a set of initial variables
    pub fn _from_file(file_path:&str) -> Self {
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



pub struct ContextStore {
    import_context:ContextSet,
    context:Vec<ContextMap>,
    local_offset:usize,
    local_size:usize,
}

impl ContextStore {
    pub fn new(import_context:ContextSet, local_offset:usize, local_size:usize) -> Self{
        Self {
            import_context,
            context:vec![ContextMap::new(local_size)],
            local_offset,
            local_size,
        }
    }

    pub fn len(&self) -> usize {
        return self.context.len() + 2;
    }

    pub fn pop_context(&mut self) {
        self.context.pop();
    }

    pub fn push_context(&mut self) {
        let lut = ContextMap::new(self.local_size);
        self.context.push(lut);
    }


    /// Returns the index of the data if it is in the context store
    fn get_data(&mut self, text:&str) -> Option<u32> {
        let l = self.context.len();
        for x in 0..l { // Search All of the Context Levels
            let result = self.context[l-x-1].get_data(text);
            if result.is_some() {
                let location = ((l-x-1)*self.local_size + self.local_offset) as u32 + result.unwrap();
                return Some(location);
            }
        }
        self.import_context.get_data(&text.to_string()).map(|s|s)
        
    }

    /// Puts the data into the context
    pub fn put_local(&mut self, text:&str, level:usize) -> u32 {
        let l = self.context.len();
        if l-level >= 1  {
            let result = self.context[l-level-1].put_data(text);
            match result {
                Some(x) => ((l-level-1)*self.local_size + self.local_offset) as u32 + x,
                None => Token::Ident.get_token_id(),
            }
        }
        else {
            Token::Ident.get_token_id()
        }
    }
    
    /// Adds the data to the context if it isn't already there
    pub fn get_or_put_local(&mut self, text:&str, level:usize) -> u32 {
        let result = self.get_data(text);
        if result.is_some() {
            result.unwrap()
        }
        else {
            self.put_local(text, level)
        }
    }

    /// Checks if data is there if not returns the default context
    pub fn read_only(&mut self, text:&str) -> u32 {
        let result = self.get_data(text);
        if result.is_some() {
            result.unwrap()
        }
        else {
            Token::Ident.get_token_id()
        }
    }

    /// Puts the data into the context
    /// write : Input is write 
    /// level : Level in the context to write (0 - top) -- used for class and def 
    pub fn put_data(&mut self, text:&str, write:bool, level:usize) -> u32 {
        if write {
            self.get_or_put_local(text, level)
        }
        else {
            self.read_only(text)
        }
    }




}

