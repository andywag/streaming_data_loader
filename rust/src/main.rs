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
    let file_path = std::fs::File::open(real_path.clone());
    let f = match file_path {
        Ok(x) => x,
        Err(_) => {
            log::error!("Can't Find File {:?}", real_path);
            std::process::exit(1);
        }
    };
    let config_file_opt = serde_yaml::from_reader(f);
    let config_file:Value = match config_file_opt {
        Ok(x) => x,
        Err(_) => {
            log::error!("Error in Config File {:?}", real_path);
            std::process::exit(1);
        }
    };
    let config_value_opt = config_file.get(real_config.clone());
    let config_ptr = match config_value_opt {
        Some(x) => Arc::new(x.to_owned()),
        None => {
            log::error!("Can't Find Config File {:?}", real_config);
            std::process::exit(1);
        }
    };

    //let config_ptr = Arc::new(config_file.get(real_config).unwrap().to_owned());
    // Run the Loader
    let result = loader::tasks::run(config_ptr["model"].as_str(), config_ptr.clone(), args.cache).await;
    log::info!("Final Result {}", result);



}