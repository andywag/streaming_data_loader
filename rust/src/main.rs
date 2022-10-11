


use loader::masking::{self};

use clap::Parser;
use serde_yaml::Value;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value="tests/masking_tests.yaml")]
   path: String,

   /// Number of times to greet
   #[arg(short, long, default_value="zmq")]
   case: String,
}



#[tokio::main] 
async fn main()  {

    let args = Args::parse();
    //println!("Args {:?}", args);
    let f = std::fs::File::open(args.path).unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    
    let _result = masking::masking_top::run_main(&config_file[args.case]).await;
    std::process::exit(0);
    
    //let base = "https://dumps.wikimedia.org/other/cirrussearch/current/";
    //let location = "/home/andy/Downloads/enwiki-20220926-cirrussearch-content.json.gz".to_string();
    
}