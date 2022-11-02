


use std::sync::Arc;

use loader::datasets::masking;
use loader::datasets::multi_label;
use loader::datasets::squad;

use clap::Parser;
use serde_yaml::Value;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value="tests/multi_label.yaml")]
   path: String,

   /// Number of times to greet
   #[arg(short, long, default_value="zmq_ipc")]
   config: String,
}



#[tokio::main] 
async fn main()  {


    let args = Args::parse();
    let f = std::fs::File::open(args.path).unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get(args.config).unwrap().to_owned());

    if true {
        let _result = multi_label::multi_runner::run(config_ptr).await;
        println!("Final Result {}", _result);
    }
    else if false {
        let _result = masking::masking_runner::run(config_ptr).await;
        println!("Final Result {}", _result);
    }
    else {
        //let result = squad::squad_top::run_main(config_ptr).await;
        let result = squad::squad_runner::run(config_ptr).await;
        println!("Squad Result {}", result);
    }
    
    
}