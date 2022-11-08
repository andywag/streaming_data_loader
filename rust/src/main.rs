use std::sync::Arc;

use clap::Parser;
use serde_yaml::Value;



/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value="tests/masking.yaml")]
   path: String,

   /// Number of times to greet
   #[arg(short, long, default_value="basic")]
   config: String,
}



#[tokio::main] 

async fn main()  {

    loader::create_logger();

    let args = Args::parse();
    let f = std::fs::File::open(args.path).unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get(args.config).unwrap().to_owned());

    let result = loader::tasks::run(config_ptr["model"].as_str(), config_ptr.clone()).await;

    log::info!("Final Result {}", result);


  
    
}