use tokenizers::Tokenizer;
use std::thread;
use std::fs;



pub fn get_tokenizer(location:String) -> Tokenizer{
    let (tx,rx)= std::sync::mpsc::channel::<Tokenizer>();
    thread::spawn(move || {
        let base = Tokenizer::from_pretrained(location, None);
        //println!("Base {:?}", base);
        let _ =tx.send(base.unwrap());
    });
    let tokenizer = rx.recv();
    return tokenizer.unwrap();   
}

pub fn store_data<T:abomonation::Abomonation>(data:&T, path:&str) {
    let mut result = Vec::<u8>::with_capacity(10000);
    unsafe {
        let _ = abomonation::encode(data, &mut result);
    }
    fs::write(path, result).unwrap();
}

pub fn compare_data<T:abomonation::Abomonation>(path:String, current:T, start:usize) -> bool {
    unsafe {
        // Load data from file
        let data = fs::read(path).unwrap(); //.as_mut_slice();
        
        // Result to place current information into
        let mut current_data = Vec::<u8>::with_capacity(data.len());
        let _ = abomonation::encode(&current, &mut current_data);

        for x in start..data.len() {
            if current_data[x] != data[x] {
                return false;
            }
        }
        
    }
    return true;
    
}

//println!("Data {:?}", data);
        