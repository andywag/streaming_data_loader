


use std::sync::Arc;

use loader::{masking::{self}, squad};

use clap::Parser;
use serde_yaml::Value;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value="tests/squad_tests.yaml")]
   path: String,

   /// Number of times to greet
   #[arg(short, long, default_value="basic")]
   config: String,
}



#[tokio::main] 
async fn main()  {

     
    //let location = "/home/andy/.cache/huggingface/datasets/squad/plain_text/1.0.0/d6ec3ceb99ca480ce37cdd35555d6cb2511d223b9150cce08a837ef62ffea453/squad-train.arrow";


    let args = Args::parse();
    //println!("Args {:?}", args);
    let f = std::fs::File::open(args.path).unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get(args.config).unwrap().to_owned());

    if false {
        let _result = masking::masking_top::run_main(config_ptr).await;
        println!("Final Result {}", _result);
    }
    else {
        let result = squad::squad_top::run_main(config_ptr).await;
        println!("Squad Result {}", result);
    }
    
    
}