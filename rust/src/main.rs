use std::sync::Arc;

use clap::Parser;
use serde_yaml::Value;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   /// Name of the person to greet
   #[arg(short, long, default_value="tests/masking.yaml")]
   path: String,
   /// Number of times to greet
   #[arg(short, long, default_value="basic")]
   config: String,

   #[arg(long, default_value=None)]
   cache: Option<String>,

}


#[tokio::main]
async fn main()  {
    loader::create_logger();

    let args = Args::parse();

    let real_path = args.path;
    let real_config = args.config; 
    // Load the Config File
    log::info!("Path {:?}", real_path);
    let f = std::fs::File::open(real_path).unwrap();
    let config_file:Value = serde_yaml::from_reader(f).unwrap();
    let config_ptr = Arc::new(config_file.get(real_config).unwrap().to_owned());
    // Run the Loader
    let result = loader::tasks::run(config_ptr["model"].as_str(), config_ptr.clone(), args.cache).await;
    log::info!("Final Result {}", result);



}