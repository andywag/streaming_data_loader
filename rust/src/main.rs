


use std::sync::Arc;

use loader::tasks::masking;
use loader::tasks::multi_label;
use loader::tasks::single_class;
use loader::tasks::squad;

use clap::Parser;
use serde_yaml::Value;



/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value="tests/single_class.yaml")]
   path: String,

   /// Number of times to greet
   #[arg(short, long, default_value="python_match")]
   config: String,
}



#[tokio::main] 

async fn main()  {

    loader::create_logger();

    let args = Args::parse();
    let f = std::fs::File::open(args.path).unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get(args.config).unwrap().to_owned());

    let result = match config_ptr["model"].as_str() {
        Some("squad") => squad::runner::run(config_ptr).await,
        Some("multi-label") => multi_label::runner::run(config_ptr).await,
        Some("single-class") => single_class::runner::run(config_ptr).await,
        Some("masking") => masking::masking_runner::run(config_ptr).await,
        Some(x) => {log::error!("Model {x} Not Found"); false}
        None => {log::error!("Model Must be specified in configuration file"); false}
    };
    log::info!("Final Result {}", result);


  
    
}