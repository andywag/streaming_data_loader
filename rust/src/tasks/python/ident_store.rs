use std::{collections::HashMap};



#[derive(Debug)]

pub struct IdentLookup {
    pub to_key:HashMap<String, usize>,
    pub from_key:HashMap<usize, String>,
    size:usize,
    current:usize
}


impl IdentLookup {
    pub fn new(s:usize) -> Self {
        Self {
            to_key: HashMap::<String,usize>::with_capacity(s),
            from_key: HashMap::<usize,String>::with_capacity(s),
            size:s,
            current:0
        }
    }

    pub fn put_data(&mut self, data:&str) -> usize {
        let result = self.to_key.get(data);
        match result {
            Some(x) => return x.clone(),
            None => {
                if self.current < self.size {
                    self.to_key.insert(data.to_string(), self.current);
                    self.from_key.insert(self.current, data.to_string()) ;
                    self.current += 1;
                    self.current - 1
                }
                else {
                    0
                }
            },
        }
    }

    pub fn get_data(&mut self, data:&str) -> Option<usize> {
        let result = self.to_key.get(data).map(|s|s.to_owned());
        result
    }

   
}

pub struct ContextStore<'a> {
    global_store:&'a mut IdentLookup,
    context:Vec<IdentLookup>
}

impl <'a>ContextStore<'a> {
    pub fn new(global_store:&'a mut IdentLookup) -> Self{
        Self {
            global_store:global_store,
            context:Vec::<IdentLookup>::with_capacity(8)
        }
    }

    pub fn len(&self) -> usize {
        return self.context.len() + 2;
    }

    pub fn pop_context(&mut self) {
        self.context.pop();
    }

    pub fn push_context(&mut self) {
        let lut = IdentLookup::new(128);
        self.context.push(lut);
    }


    pub fn get_data(&mut self, text:&str) -> Option<(usize,usize)> {
        let l = self.context.len();
        for x in 0..l { // Search All of the Context Levels
            let result = self.context[l-x-1].get_data(text);
            if result.is_some() {
                return Some((l-x-1,result.unwrap()));
            }
        }
        let glob = self.global_store.get_data(text);
        glob.map(|s|(0,s))
    }

    // Put identifier onto the global context of file context if doesn't fit
    pub fn put_local(&mut self, text:&str) -> (usize,usize) {
        if self.context.len() > 0 {
            let l = self.context.len();
            (l-1, self.context[l-1].put_data(text))
        }
        else {
            (0, self.global_store.put_data(text))
        }
        
        
    }



    pub fn get_or_global(&mut self, text:&str) -> (usize,usize) {
        match self.get_data(text) {
            Some(x) => x,
            None => (0,self.global_store.put_data(text)),
        }
    }


}
